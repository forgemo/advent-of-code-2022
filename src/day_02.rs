use std::fs;
use itertools::Itertools;
use crate::Outcome::{Draw, Lost, Won};
use crate::Shape::{Paper, Rock, Scissor};

fn main() {
    let input = fs::read_to_string("input/day_02.txt").unwrap();
    let lines = input.split('\n').collect_vec();

    let solution_1: usize = lines.iter()
        .map(|round| round
            .split(' ')
            .map(Shape::from)
            .next_tuple::<(_, _)>().unwrap()
        ).map(Round::from)
        .map(|r|r.score())
        .sum();


    let solution_2: usize = lines.iter()
        .map(|round| round.split(' ').next_tuple::<(_, _)>().unwrap())
        .map(|(left, right)| (Shape::from(left), Outcome::from(right)) )
        .map(|(opponent, outcome)| Round::from_outcome(opponent, outcome))
        .map(|round|round.score())
        .sum();

    assert_eq!(solution_1, 9177);
    assert_eq!(solution_2, 12111);

}


enum Shape { Rock, Paper, Scissor }
enum Outcome { Lost, Won, Draw }
struct Round (Shape, Shape);

impl From<&str> for Shape {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissor,
            _ => panic!("parsing input {} failed", s)
        }
    }
}

impl Shape {
    fn against(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Rock, Rock) => Draw,
            (Rock, Paper) => Lost,
            (Rock, Scissor) => Won,
            (Paper, Rock) => Won,
            (Paper, Paper) => Draw,
            (Paper, Scissor) => Lost,
            (Scissor, Rock) => Lost,
            (Scissor, Paper) => Won,
            (Scissor, Scissor) => Draw,
        }
    }

    fn from_outcome(other: &Shape, outcome: &Outcome) -> Self {
        match (other, outcome) {
            (Rock, Lost) => Scissor,
            (Rock, Draw) => Rock,
            (Rock, Won) => Paper,
            (Paper, Lost) => Rock,
            (Paper, Draw) => Paper,
            (Paper, Won) => Scissor,
            (Scissor, Lost) => Paper,
            (Scissor, Draw) => Scissor,
            (Scissor, Won) => Rock,
        }
    }

    fn value(&self) -> usize {
        match self {
            Rock => 1,
            Paper => 2,
            Scissor => 3,
        }
    }
}

impl From<(Shape, Shape)> for Round {
    fn from((left, right): (Shape, Shape)) -> Self {
        Round(left, right)
    }
}

impl Outcome {
    fn value(&self) -> usize {
        match self {
            Won => 6,
            Draw => 3,
            Lost => 0,
        }
    }
}

impl From<&str> for Outcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => Lost,
            "Y" => Draw,
            "Z" => Won,
            _ => panic!("parsing input {} failed", s)
        }
    }
}


impl Round {

    fn from_outcome(opponent: Shape, outcome: Outcome) -> Self {
        let needed_shape = Shape::from_outcome(&opponent, &outcome);
        Round(opponent, needed_shape)
    }

    fn score(&self) -> usize {
        let Round(left, right) = self;
        let outcome = right.against(left);
        outcome.value() + right.value()
    }
}