use std::collections::HashSet;
use std::fs;
use itertools::Itertools;

fn main() {

    let input = fs::read_to_string("input/day_06.txt").unwrap();
    let solution_1 = find_first_marker(&input, 4);
    let solution_2 = find_first_marker(&input, 14);

    assert_eq!(solution_1, 1356);
    assert_eq!(solution_2, 2564);
}

fn find_first_marker(input: &str, len: usize) -> usize {
    input.chars().collect_vec()
        .windows(len)
        .map(|chars| chars.iter().collect::<HashSet<&char>>())
        .enumerate()
        .find(|(_, set)| set.len() == len)
        .map(|(index, _)| index + len)
        .unwrap()
}