use std::ops::{Add, Sub};
use itertools::Itertools;
use pathfinding::prelude::{astar};
use rayon::prelude::*;
use crate::Action::{BuildRobot, Wait};

fn main() {
    rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();

    let context_1 = build_context(24);
    let solution_1 = solve(context_1)
        .map(|(id, geode)| (id + 1) * geode)
        .sum::<usize>();
    assert_eq!(solution_1, 1177);


    let context_2 = build_context(32).with_first_n_blueprints(3);
    let solution_2 = solve(context_2)
        .map(|(_, geode)| geode)
        .product::<usize>();
    assert_eq!(solution_2, 62744);
}

#[derive(Clone)]
struct Context {
    blueprints: Vec<Vec<Robot>>,
    selected_blueprint: usize,
    max_minutes: usize,
}

impl Context {
    fn selected_blueprint(&self) -> &Vec<Robot> {
        &self.blueprints[self.selected_blueprint]
    }

    fn with_selected_blueprint(&self, index: usize) -> Self {
        Self {
            selected_blueprint: index,
            ..self.clone()
        }
    }

    fn with_first_n_blueprints(&self, n: usize) -> Self {
        Self {
            blueprints: self.blueprints.iter().take(n).cloned().collect(),
            ..self.clone()
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    rpm: Resources,
    after_minute: usize,
    available: Resources,
}

impl Default for State {
    fn default() -> Self {
        State { rpm: Resources::from_ore(1), available: Default::default(), after_minute: 0 }
    }
}

impl State {
    fn successors(&self, ctx: &Context) -> Vec<State> {
        if self.after_minute == ctx.max_minutes {
            return Vec::default();
        }

        let mut actions = ctx.selected_blueprint().iter()
            .filter(|robot| self.available.enough_for(&robot.cost))
            .map(Action::BuildRobot)
            .collect_vec();
        if actions.len() < ctx.blueprints.len() {
            actions.push(Wait);
        }

        actions.iter()
            .map(|action| {
                match action {
                    BuildRobot(r) => {
                        Self {
                            available: &(&self.available + &self.rpm) - &r.cost,
                            rpm: &self.rpm + &r.rpm,
                            after_minute: self.after_minute + 1,
                        }
                    }
                    Wait => {
                        Self {
                            available: &self.available + &self.rpm,
                            after_minute: self.after_minute + 1,
                            ..self.clone()
                        }
                    }
                }
            })
            .collect_vec()
    }
}

#[derive(Debug)]
enum Action<'a> {
    BuildRobot(&'a Robot),
    Wait,
}

#[derive(Clone, PartialOrd, PartialEq, Eq, Hash, Default, Debug)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Resources {
    fn from_ore(ore: usize) -> Self { Self { ore, ..Default::default() } }
    fn from_clay(clay: usize) -> Self { Self { clay, ..Default::default() } }
    fn from_ore_and_clay(ore: usize, clay: usize) -> Self { Self { ore, clay, ..Default::default() } }
    fn from_obsidian(obsidian: usize) -> Self { Self { obsidian, ..Default::default() } }
    fn from_ore_and_obsidian(ore: usize, obsidian: usize) -> Self { Self { ore, obsidian, ..Default::default() } }
    fn from_geode(geode: usize) -> Self { Self { geode, ..Default::default() } }
    fn enough_for(&self, rhs: &Resources) -> bool {
        self.ore >= rhs.ore && self.clay >= rhs.clay && self.obsidian >= rhs.obsidian && self.geode >= rhs.geode
    }
}

#[derive(Clone, Debug)]
struct Robot {
    cost: Resources,
    rpm: Resources,
}

impl Robot {
    fn new_ore_robot(ore_cost: usize) -> Self {
        Self {
            cost: Resources::from_ore(ore_cost),
            rpm: Resources::from_ore(1),
        }
    }
    fn new_clay_robot(ore_cost: usize) -> Self {
        Self {
            cost: Resources::from_ore(ore_cost),
            rpm: Resources::from_clay(1),
        }
    }
    fn new_obsidian_robot(ore_cost: usize, clay_cost: usize) -> Self {
        Self {
            cost: Resources::from_ore_and_clay(ore_cost, clay_cost),
            rpm: Resources::from_obsidian(1),
        }
    }
    fn new_geode_robot(ore_cost: usize, obsidian_cost: usize) -> Self {
        Self {
            cost: Resources::from_ore_and_obsidian(ore_cost, obsidian_cost),
            rpm: Resources::from_geode(1),
        }
    }
}


impl Sub for &Resources {
    type Output = Resources;

    fn sub(self, rhs: Self) -> Self::Output {
        Resources {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl Add for &Resources {
    type Output = Resources;

    fn add(self, rhs: Self) -> Self::Output {
        Resources {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

fn solve(ctx: Context) -> impl ParallelIterator<Item=(usize, usize)> {
    (0..ctx.blueprints.len()).collect_vec()
        .into_par_iter()
        .map(move |i| ctx.with_selected_blueprint(i))
        .map(|ctx| {
            let (path, _) = astar(
                &State::default(),
                |s| s.successors(&ctx).into_iter().map(|s| {
                    let cost = 200 - s.rpm.geode;
                    (s, cost)
                }),
                |s| {
                    let remaining_minutes = ctx.max_minutes - s.after_minute;
                    (1..remaining_minutes + 1).map(|i| 200 - (s.rpm.geode + i)).sum::<usize>()
                },
                |s| s.after_minute == ctx.max_minutes,
            ).unwrap();
            //println!("{:#?}", path);
            (ctx.selected_blueprint, path.last().unwrap().available.geode)
        })
        .inspect(|(id, geode)| println!("id: {} => {} geodes", id, geode))
}


fn build_blueprint(cost: [usize; 6]) -> Vec<Robot> {
    vec![Robot::new_ore_robot(cost[0]),
         Robot::new_clay_robot(cost[1]),
         Robot::new_obsidian_robot(cost[2], cost[3]),
         Robot::new_geode_robot(cost[4], cost[5])]
}

fn build_context(max_minutes: usize) -> Context {
    let blueprints = vec![
        [3, 3, 3, 15, 2, 8],
        [2, 3, 3, 17, 3, 10],
        [2, 2, 2, 20, 2, 14],
        [4, 4, 3, 14, 4, 15],
        [2, 3, 3, 13, 3, 15],
        [2, 2, 2, 15, 2, 7],
        [3, 3, 3, 9, 3, 7],
        [4, 2, 2, 16, 2, 8],
        [2, 4, 4, 20, 4, 18],
        [3, 3, 2, 11, 2, 19],
        [4, 4, 2, 7, 3, 10],
        [2, 3, 3, 11, 2, 16],
        [3, 4, 4, 16, 3, 15],
        [4, 3, 4, 18, 3, 13],
        [2, 3, 3, 13, 2, 20],
        [3, 4, 4, 14, 4, 10],
        [4, 3, 2, 17, 3, 16],
        [2, 4, 3, 20, 2, 17],
        [2, 4, 2, 16, 4, 12],
        [3, 3, 3, 16, 3, 20],
        [3, 4, 4, 18, 4, 12],
        [3, 4, 3, 13, 3, 19],
        [3, 4, 4, 18, 3, 8],
        [4, 3, 2, 13, 2, 9],
        [4, 4, 4, 5, 3, 15],
        [4, 4, 2, 15, 3, 16],
        [3, 4, 4, 20, 4, 16],
        [4, 3, 4, 8, 2, 8],
        [4, 4, 2, 14, 4, 19],
        [3, 4, 3, 10, 2, 7],
    ].into_iter().map(build_blueprint).collect();

    Context {
        blueprints,
        selected_blueprint: 0,
        max_minutes,
    }
}