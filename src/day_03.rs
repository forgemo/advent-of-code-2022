use std::collections::BTreeSet;
use std::fs;
use itertools::Itertools;
use tuple::Map;

fn main() {
    let input = fs::read_to_string("input/day_03.txt").unwrap();

    let solution_1 = input.split('\n')
        .map(|line| line.split_at(line.len()/2))
        .map(|tuple| tuple.map(|s| BTreeSet::from_iter(s.chars())))
        .map(|(left, right)| left.intersection(&right).cloned().collect_vec())
        .map(|items| items.iter().map(item_priority).sum::<usize>())
        .sum::<usize>();

    assert_eq!(7568, solution_1);


    let solution_2 = input.split('\n')
        .tuples::<(_, _,_)>()
        .map(|tuple| tuple.map(|s| BTreeSet::from_iter(s.chars())))
        .map(|(a, b, c)|
            BTreeSet::from_iter(a.intersection(&b).cloned()).intersection(&c).cloned().collect_vec()
        )
        .map(|items| items.first().map(item_priority).unwrap())
        .sum::<usize>();

    assert_eq!(2780, solution_2);
}

fn item_priority(item: &char) -> usize {
    if item.is_lowercase() {
        *item as usize - 96
    } else {
        *item as usize - 38
    }
}