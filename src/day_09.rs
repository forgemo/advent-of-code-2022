use std::collections::HashSet;
use std::fs;
use itertools::Itertools;
use crate::Direction::*;

fn main() {
    let input = fs::read_to_string("input/day_09.txt").unwrap();

    let moves = input.split('\n')
        .map(|line| line.split(' ').next_tuple::<(_, _)>().unwrap())
        .map(Move::from).collect_vec();

    let rope_1 = (0..2).map(|_| Pos::zero()).collect_vec();
    let solution_1 = position_count_for_rope(rope_1, &moves);

    let rope_2 = (0..10).map(|_| Pos::zero()).collect_vec();
    let solution_2 = position_count_for_rope(rope_2, &moves);

    assert_eq!(solution_1, 5710);
    assert_eq!(solution_2, 2259);
}


fn position_count_for_rope(mut rope: Vec<Pos>, moves: &[Move]) -> usize {
    let mut positions = HashSet::new();

    positions.insert(rope[0]);

    moves.iter().for_each(|m| {
        for _ in 0..m.steps {
            rope[0] = m.as_single_step().apply(&rope[0]);

            (0..rope.len() - 1).for_each(|i| {
                rope[i + 1] = rope[i + 1].follow(&rope[i]);
            });
            positions.insert(rope[rope.len() - 1]);
        }
    });

    positions.len()
}

#[derive(Copy, Clone, Debug)]
enum Direction { Right, Left, Up, Down }

#[derive(Copy, Clone, Debug)]
struct Move {
    direction: Direction,
    steps: usize,
}

impl Move {
    fn apply(&self, pos: &Pos) -> Pos {
        match self.direction {
            Right => pos.move_right(self.steps),
            Left => pos.move_left(self.steps),
            Up => pos.move_up(self.steps),
            Down => pos.move_down(self.steps),
        }
    }

    fn as_single_step(&self) -> Self {
        Move {
            direction: self.direction,
            steps: 1,
        }
    }
}

impl From<(&str, &str)> for Move {
    fn from(dir_steps: (&str, &str)) -> Self {
        let (dir, steps) = dir_steps;
        Move {
            direction: match dir {
                "R" => Right,
                "L" => Left,
                "U" => Up,
                "D" => Down,
                _ => unimplemented!()
            },
            steps: steps.parse::<usize>().unwrap(),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialOrd, PartialEq, Ord, Eq)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn new(x: isize, y: isize) -> Self { Pos { x, y } }
    fn zero() -> Self {
        Pos::new(0, 0)
    }
    fn move_right(&self, steps: usize) -> Self {
        Self::new(self.x + steps as isize, self.y)
    }
    fn move_left(&self, steps: usize) -> Self {
        Self::new(self.x - steps as isize, self.y)
    }
    fn move_up(&self, steps: usize) -> Self {
        Self::new(self.x, self.y - steps as isize)
    }
    fn move_down(&self, steps: usize) -> Self {
        Self::new(self.x, self.y + steps as isize)
    }
    fn dist(&self, head: &Pos) -> Self { Pos::new(head.x - self.x, head.y - self.y) }
    fn follow(&self, head: &Pos) -> Self {
        let dist = self.dist(head);
       if dist.x.abs() >= 2 || dist.y.abs() >= 2 {
           Pos::new(self.x + dist.x.signum(), self.y + dist.y.signum())
        } else {
            *self
       }
    }
}