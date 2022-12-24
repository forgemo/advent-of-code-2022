extern crate core;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use itertools::{Itertools};
use pathfinding::prelude::{astar};

use crate::Action::{MoveTo, OpenValve, Wait};

fn main() {
    { // sample
        let input_sample = fs::read_to_string("input/day_16_sample.txt").unwrap();
        let arena = parse_input(&input_sample);
        let start_valve = &arena["AA"];

        let context = Context::new(&arena, 30);

        let start = State::new(start_valve);

        let solution_1 = solve(start.clone(), &context, true);
        assert_eq!(solution_1, 1651);

        let solution_2 = solve(start.clone(), &context.with_max_minutes(26), false);
        assert_eq!(solution_2, 1707);
    }


    { // real input
        let input = fs::read_to_string("input/day_16.txt").unwrap();

        let arena = parse_input(&input);
        let start_valve = &arena["AA"];

        let context = Context::new(&arena, 30);

        let start = State::new(start_valve);

        let solution_1 = solve(start.clone(), &context, true);
        assert_eq!(solution_1, 1617);

        let solution_2 = solve(start, &context.with_max_minutes(26), false);
        assert_eq!(solution_2, 2171);
    }
}

#[derive(Clone)]
struct Context<'a> {
    arena: &'a BTreeMap<String, Valve>,
    move_action_map: BTreeMap<&'a Valve, Vec<Action<'a>>>,
    ignore_valves: BTreeSet<&'a Valve>,
    relevant_valves: BTreeSet<&'a Valve>,
    max_minutes: usize,
}

impl<'a> Context<'a> {
    fn new(arena: &'a BTreeMap<String, Valve>, max_minutes: usize) -> Self {
        let move_action_map = build_move_action_map(arena);
        let (relevant_valves, ignore_valves) = arena.values().partition(|v| v.flow_rate > 0);

        Context {
            arena,
            move_action_map,
            ignore_valves,
            relevant_valves,
            max_minutes,
        }
    }

    fn with_max_minutes(&self, max_minutes: usize) -> Self {
        Self {
            max_minutes,
            ..self.clone()
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct State<'a> {
    human_position: &'a Valve,
    elephant_position: &'a Valve,
    open_valves: BTreeSet<&'a Valve>,
    after_minute: usize,
    pressure_rpm: usize,
    total_released_pressure: usize,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
enum Action<'a> {
    OpenValve(&'a Valve),
    MoveTo(&'a Valve),
    Wait,
}

impl<'a> State<'a> {
    fn new(start_valve: &'a Valve) -> Self {
        Self {
            human_position: start_valve,
            elephant_position: start_valve,
            open_valves: Default::default(),
            after_minute: 0,
            pressure_rpm: 0,
            total_released_pressure: 0,
        }
    }

    fn possible_actions(&self, context: &'a Context<'a>, skip_elephant: bool) -> Vec<(Action<'a>, Action<'a>)> {
        if self.after_minute == context.max_minutes {
            return vec![];
        }

        let all_valves_open = self.open_valves.len() == context.arena.len() - context.ignore_valves.len();
        if all_valves_open {
            return vec![(Wait, Wait)];
        }

        let mut human_moves = context.move_action_map.get(&self.human_position).unwrap().iter().cloned().collect_vec();
        if !self.open_valves.contains(&self.human_position) && !context.ignore_valves.contains(&self.human_position) {
            human_moves.push(OpenValve(self.human_position))
        }

        let mut elephant_moves = vec![];
        if skip_elephant {
            elephant_moves.push(Wait);
        } else {
            elephant_moves.extend(context.move_action_map.get(&self.elephant_position).unwrap().iter().cloned());
            if !self.open_valves.contains(&self.elephant_position) && !context.ignore_valves.contains(&self.elephant_position) {
                elephant_moves.push(OpenValve(self.elephant_position))
            }
        };

        let actions = human_moves.iter().flat_map(|human| elephant_moves.iter().map(|elephant| (human.clone(), elephant.clone())))
            .filter(|tuple| !matches!(tuple, (OpenValve(h), OpenValve(e)) if h == e))
            .collect_vec();
        actions
    }

    fn move_elephant_to(mut self, v: &'a Valve) -> Self {
        self.elephant_position = v;
        self
    }
    fn move_human_to(mut self, v: &'a Valve) -> Self {
        self.human_position = v;
        self
    }
    fn open_valve(mut self, v: &'a Valve) -> Self {
        self.open_valves.insert(v);
        self.pressure_rpm += v.flow_rate;
        self
    }
    fn add_minute(mut self) -> Self {
        self.after_minute += 1;
        self
    }
    fn fast_forward(mut self, context: &Context) -> Self {
        let remaining_minutes = context.max_minutes - self.after_minute;
        self.total_released_pressure += self.pressure_rpm * remaining_minutes;
        self.after_minute = context.max_minutes;
        self
    }

    fn successors(&self, context: &'a Context<'a>, skip_elephant: bool) -> Vec<State<'a>> {
        let mut temp = self.clone();
        temp.total_released_pressure += temp.pressure_rpm;

        temp.possible_actions(context, skip_elephant)
            .into_iter()
            .map(|actions| {
                match actions {
                    (OpenValve(h), OpenValve(e)) =>
                        temp.clone().open_valve(h).open_valve(e).add_minute(),
                    (MoveTo(h), MoveTo(e)) =>
                        temp.clone().move_human_to(h).move_elephant_to(e).add_minute(),
                    (Wait, Wait) =>
                        temp.clone().add_minute().fast_forward(context),
                    (OpenValve(h), MoveTo(e)) =>
                        temp.clone().open_valve(h).move_elephant_to(e).add_minute(),
                    (MoveTo(h), OpenValve(e)) =>
                        temp.clone().move_human_to(h).open_valve(e).add_minute(),
                    (OpenValve(v), Wait) =>
                        temp.clone().open_valve(v).add_minute(),
                    (MoveTo(v), Wait) =>
                        temp.clone().move_human_to(v).add_minute(),
                    (Wait, OpenValve(v)) =>
                        temp.clone().open_valve(v).add_minute(),
                    (Wait, MoveTo(v)) =>
                        temp.clone().move_elephant_to(v).add_minute()
                }
            })
            .collect()
    }

}


fn build_move_action_map(arena: &BTreeMap<String, Valve>) -> BTreeMap<&Valve, Vec<Action>> {
    arena.values().map(|v| {
        let actions = v.tunnels_to.iter().map(|label| Action::MoveTo(&arena[label])).collect_vec();
        (v, actions)
    }).collect()
}

type Label = String;

#[derive(Hash, Eq, PartialEq, Debug, Ord, PartialOrd)]
struct Valve {
    label: Label,
    flow_rate: usize,
    tunnels_to: BTreeSet<String>,
}

fn solve<'a>(start: State, context: &'a Context<'a>, skip_elephant: bool) -> usize {
    let best_rpm = context.arena.values().map(|v| v.flow_rate).sum::<usize>();

    let (path, _) = astar(&start,
                          |s| {
                              s.successors(context, skip_elephant).into_iter().map(move |succ| {
                                  let cost = (context.max_minutes + 1 - succ.after_minute) * (best_rpm - succ.pressure_rpm);
                                  (succ, cost)
                              }).collect_vec()
                          },
                          |s| {
                              let minutes_left = context.max_minutes + 1 - s.after_minute;
                              let closed_valves = context.relevant_valves.difference(&s.open_valves).map(|v| v.flow_rate).sorted().rev().take(minutes_left).collect_vec();
                              let potential_rpm_of_top_closed = closed_valves.iter().sum::<usize>(); // todo optimize
                              best_rpm - potential_rpm_of_top_closed
                          },
                          |s| s.after_minute == context.max_minutes,
    ).unwrap();

    path.last().unwrap().total_released_pressure
}

fn parse_input(input: &str) -> BTreeMap<String, Valve> {
    let valves: BTreeMap<String, Valve> = input.lines().map(|l| {
        let label = l.split(' ').nth(1).unwrap().to_string();
        let flow_rate = l.split('=').nth(1).unwrap().split(';').next().unwrap().parse::<usize>().unwrap();
        let tunnels_to = l.replace(',', "")
            .split(' ')
            .skip_while(|w| !w.contains("valve"))
            .skip(1).map(|s| s.to_string())
            .collect();

        let valve = Valve {
            label,
            flow_rate,
            tunnels_to,
        };
        (valve.label.clone(), valve)
    }).collect();
    valves
}
