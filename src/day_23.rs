use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use std::fs;
use std::ops::{Add, Sub};
use std::str::FromStr;
use itertools::Itertools;
use crate::Direction::*;

fn main() {
    let input = fs::read_to_string("input/day_23.txt").unwrap();

    let mut world = input.parse::<World>().unwrap();
    world.next_n_rounds(10);
    assert_eq!(world.empty_tiles_in_smallest_rect(), 3931);

    let mut world = input.parse::<World>().unwrap();
    let rounds = world.simulate_until_no_movement();
    assert_eq!(rounds, 944);
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Direction { North, South, West, East }


#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self { Position { x, y } }
    fn adjacent(&self) -> BTreeSet<Position> {
        [(-1,-1),(0,-1),(1,-1),(-1,0),(1,0),(-1,1),(0,1),(1,1)]
            .into_iter()
            .map(|d|self+d).collect()
    }
    fn adjacent_at_with_diag(&self, direction: &Direction) -> BTreeSet<Position> {
        match direction {
            North => BTreeSet::from([self - (1, 1), self - (0, 1), self + (1, -1)]),
            South => BTreeSet::from([self + (-1, 1), self + (0, 1), self + (1, 1)]),
            West => BTreeSet::from([self - (1, 1), self - (1, 0), self + (-1, 1)]),
            East => BTreeSet::from([self + (1, -1), self + (1, 0), self + (1, 1)]),
        }
    }
    fn adjacent_at(&self, direction: &Direction) -> Position {
        match direction {
            North => self - (0, 1),
            South => self + (0, 1),
            West => self - (1, 0),
            East => self + (1, 0),
        }
    }
}

impl Sub<(isize, isize)> for &Position {
    type Output = Position;

    fn sub(self, rhs: (isize, isize)) -> Self::Output {
        Position::new(self.x - rhs.0, self.y - rhs.1)
    }
}

impl Add<(isize, isize)> for &Position {
    type Output = Position;

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        Position::new(self.x + rhs.0, self.y + rhs.1)
    }
}

struct World {
    elves: BTreeSet<Position>,
    movement_order: Vec<Direction>,
}


impl World {
    fn next_round(&mut self) -> bool{
        let moving_elves = self.elves.iter()
            .filter(|e| !e.adjacent().is_disjoint(&self.elves))
            .collect_vec();
        let moves = moving_elves.into_iter()
            .filter_map(|e| self.movement_order.iter()
                .filter(|direction| e.adjacent_at_with_diag(direction).is_disjoint(&self.elves))
                .map(|direction| (e, e.adjacent_at(direction)))
                .next()
            )
            .sorted_by_key(|(_, to)| to.clone())
            .dedup_by_with_count(|a, b| a.1 == b.1)
            .filter(|(count, _)| *count == 1)
            .map(|(_, proposal)| (proposal.0.clone(), proposal.1))
            .collect_vec();

        let any_moves = !moves.is_empty();
        moves.into_iter().for_each(|(from, to)| {
            self.elves.remove(&from);
            self.elves.insert(to);
        });

        let direction = self.movement_order.remove(0);
        self.movement_order.push(direction);

        any_moves
    }

    fn next_n_rounds(&mut self, n: usize) {
        (0..n).for_each(|_| {self.next_round();})
    }

    fn empty_tiles_in_smallest_rect(&self) -> usize {
        let x_range = self.elves.iter().map(|e| e.x).minmax().into_option().unwrap();
        let y_range = self.elves.iter().map(|e| e.y).minmax().into_option().unwrap();
        ((x_range.1 - x_range.0 + 1) * (y_range.1 - y_range.0 + 1)) as usize - self.elves.len()
    }

    fn simulate_until_no_movement(&mut self) -> usize {
        let mut count = 1;
        while self.next_round() {count+=1}
        count
    }
}

impl FromStr for World {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elves = s.lines().enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate()
                    .filter(|(_, c)| c == &'#')
                    .map(move |(x, _)| Position::new(x as isize, y as isize))
            }).collect();

        let movement_order = vec![North, South, West, East];
        Ok(Self { elves, movement_order })
    }
}

impl Display for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let x_range = self.elves.iter().map(|e| e.x).minmax().into_option().unwrap();
        let y_range = self.elves.iter().map(|e| e.y).minmax().into_option().unwrap();
        for y in y_range.0..y_range.1+1 {
            for x in x_range.0..x_range.1+1 {
                let pos = Position::new(x,y);
                if self.elves.contains(&pos) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}