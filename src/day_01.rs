use std::fs;
use itertools::Itertools;

fn main() {

    let input = fs::read_to_string("input/day_01.txt").unwrap();

    let calories_per_elve = input
        .split("\n\n")
        .map(|elve|elve
            .split("\n")
            .map(|s|s.parse::<usize>().unwrap()).sum()
        ).collect_vec();

    let solution_1 = *calories_per_elve.iter().max().unwrap();
    let solution_2 = calories_per_elve.iter().sorted().rev().take(3).sum::<usize>();

    assert_eq!(solution_1, 72602);
    assert_eq!(solution_2, 207410);
}
