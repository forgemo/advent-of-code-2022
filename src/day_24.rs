use std::collections::{BTreeSet};
use std::fmt::{Display, Formatter};
use std::fs;
use std::str::FromStr;
use itertools::Itertools;
use pathfinding::prelude::astar;
use crate::Action::{Move, Wait};
use crate::Direction::*;

fn main() {
    let input = fs::read_to_string("input/day_24.txt").unwrap();
    let context = input.parse::<Context>().unwrap();
    let solution_1 = solve_1(context.clone());
    assert_eq!(solution_1, 247);
    let solution_2 = solve_2(context);
    assert_eq!(solution_2, 728);
}

enum Action { Move(Direction), Wait }

impl Action {
    fn all() -> [Action; 5] {
        [Move(Left), Move(Right), Move(Up), Move(Down), Wait]
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction { Left, Right, Up, Down }

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
    fn adjacent_at(&self, direction: &Direction) -> Self {
        match direction {
            Left => Position::new(self.x - 1, self.y),
            Right => Position::new(self.x + 1, self.y),
            Up => Position::new(self.x, self.y - 1),
            Down => Position::new(self.x, self.y + 1),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Blizzard {
    position: Position,
    direction: Direction,
}

impl Blizzard {
    fn with_position(&self, position: Position) -> Self {
        Blizzard { position, direction: self.direction.clone() }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Context {
    height: usize,
    width: usize,
    entry: Position,
    exit: Position,
    walls: BTreeSet<Position>,
    initial_blizzards: Vec<Blizzard>,
}

impl Context {
    fn is_wall(&self, pos: &Position) -> bool {
        self.walls.contains(pos)
    }

    fn blizzard_energy_conservation(&self, pos: &Position) -> Position {
        match (pos.x, pos.y) {
            (x, y) if x == 0 => Position::new(self.width - 2, y),
            (x, y) if x == self.width - 1 => Position::new(1, y),
            (x, y) if y == 0 => Position::new(x, self.height - 2),
            (x, y) if y == self.height - 1 => Position::new(x, 1),
            _ => unimplemented!("no energy conservation needed for {:?}", pos)
        }
    }

    fn with_swapped_entry_exit(&self) -> Self {
        Context {
            entry: self.exit.clone(),
            exit: self.entry.clone(),
            ..self.clone()}
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    expedition: Position,
    blizzard_positions: BTreeSet<Position>,
    blizzards: Vec<Blizzard>,
}


impl State {
    fn successors(&self, context: &Context) -> Vec<Self> {
        let mut template = self.clone();
        template.move_blizzards(context);
        Action::all().iter()
            .filter_map(|action| {
                match action {
                    Move(Up) if self.expedition.y == 0 => None,
                    Move(direction) => {
                        let pos = self.expedition.adjacent_at(direction);
                        if context.is_wall(&pos) || template.blizzard_positions.contains(&pos) {
                            None
                        } else {
                            Some(pos)
                        }
                    }
                    Wait => if template.blizzard_positions.contains(&self.expedition) {
                        None
                    } else {
                        Some(self.expedition.clone())
                    }
                }
            })
            .map(|expedition| template.with_expedition(expedition))
            .collect_vec()
    }

    fn move_blizzards(&mut self, context: &Context) {
        self.blizzards = self.blizzards.iter().map(|b| {
            let next_pos = b.position.adjacent_at(&b.direction);
            if context.is_wall(&next_pos) {
                let pos = context.blizzard_energy_conservation(&next_pos);
                b.with_position(pos)
            } else {
                b.with_position(next_pos)
            }
        }).collect();
        self.blizzard_positions = self.blizzards.iter()
            .map(|b| b.position.clone())
            .collect()
    }

    fn with_expedition(&self, expedition: Position) -> State {
        State { expedition, ..self.clone() }
    }

    fn distance_to_exit(&self, context: &Context) -> usize {
        ((context.exit.x.abs_diff(self.expedition.x).pow(2)
            + context.exit.y.abs_diff(self.expedition.y).pow(2)
        ) as f64).sqrt() as usize
    }

    fn from_context(context: &Context) -> Self {
        State {
            expedition: context.entry.clone(),
            blizzards: context.initial_blizzards.clone(),
            blizzard_positions: context.initial_blizzards.iter().map(|b| b.position.clone()).collect(),
        }
    }
}


impl FromStr for Context {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut walls = BTreeSet::new();
        let mut blizzards = vec![];
        s.lines()
            .enumerate()
            .flat_map(|(y, line)|
                line.chars().enumerate().map(move |(x, c)| (Position::new(x, y), c)))
            .for_each(|(position, c)| {
                match c {
                    '.' => {}
                    '#' => { walls.insert(position); }
                    '>' => blizzards.push(Blizzard { position, direction: Right }),
                    '<' => blizzards.push(Blizzard { position, direction: Left }),
                    'v' => blizzards.push(Blizzard { position, direction: Down }),
                    '^' => blizzards.push(Blizzard { position, direction: Up }),
                    _ => unimplemented!("unexpected tile {}", c)
                }
            });
        let height = walls.iter().map(|p| p.y).max().unwrap() + 1;
        let width = walls.iter().map(|p| p.x).max().unwrap() + 1;
        let entry = (0..width).map(|x| Position::new(x, 0)).find(|pos| !walls.contains(pos)).unwrap();
        let exit = (0..width).map(|x| Position::new(x, height - 1)).find(|pos| !walls.contains(pos)).unwrap();
        Ok(Context {
            height,
            width,
            entry,
            exit,
            walls,
            initial_blizzards: blizzards,
        })
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Left => write!(f, "<"),
            Right => write!(f, ">"),
            Up => write!(f, "^"),
            Down => write!(f, "v"),
        }
    }
}

fn print_state(state: &State, context: &Context) {
    (0..context.height).for_each(|y| {
        (0..context.width).for_each(|x| {
            let p = Position::new(x, y);
            if state.expedition == p {
                print!("E")
            } else if context.walls.contains(&p) {
                print!("#")
            } else if state.blizzard_positions.contains(&p) {
                let blizzards = state.blizzards.iter().filter(|b|b.position == p).collect_vec();
                if blizzards.len() == 1 {
                    print!("{}", blizzards[0].direction)
                } else {
                    print!("{}", blizzards.len())
                }
            } else {
                print!(".")
            }
        });
        println!()
    });
}

fn solve(initial_state: &State, context: Context) -> (Vec<State>, usize) {
    astar(initial_state,
                          |s| s.successors(&context).into_iter().map(|s| (s, 1)).collect_vec(),
                          |s| s.distance_to_exit(&context),
                          |s| s.expedition == context.exit,
    ).unwrap()
}

fn solve_1(context: Context) -> usize {
    solve(&State::from_context(&context), context).1
}

fn solve_2(ctx_to_exit: Context) -> usize {
    let ctx_to_entry = ctx_to_exit.with_swapped_entry_exit();

    let start = State::from_context(&ctx_to_exit);
    let (path_1, cost_1) = solve(&start, ctx_to_exit.clone());
    println!("cost 1: {}", cost_1);

    let start = path_1.last().unwrap();
    let (path_2, cost_2) = solve(start, ctx_to_entry);
    println!("cost 2: {}", cost_2);

    let start = path_2.last().unwrap();
    let (_, cost_3) = solve(start, ctx_to_exit);
    println!("cost 3: {}", cost_3);

    cost_1 + cost_2 + cost_3
}