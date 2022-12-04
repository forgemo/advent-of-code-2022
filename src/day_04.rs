use std::fs;
use itertools::Itertools;

fn main() {
    let assignments = fs::read_to_string("input/day_04.txt").unwrap()
        .split('\n')
        .map(|line|line.split(',')
            .map(|range|range.split('-')
                .map(|value| value.parse::<usize>().unwrap())
                .tuples::<(_, _)>().next().unwrap()))
        .map(|pair| pair.tuples::<(_,_)>().next().unwrap())
        .collect_vec();

    let solution_1 = assignments.iter()
        .filter(|(l, r)| contains(l,r) || contains(r, l))
        .count();

    let solution_2 = assignments.iter()
        .filter(overlap_at_all)
        .count();

    assert_eq!(solution_1, 450);
    assert_eq!(solution_2, 837);
}


fn contains(l: &(usize, usize), r: &(usize, usize)) -> bool {
    let (l1, l2) = l;
    let (r1, r2) = r;
    r1 >= l1 && r2 <= l2
}

fn overlap_at_all(range: &&((usize, usize), (usize, usize))) -> bool {
    let ((l1, l2), (r1, r2)) = range;
    l1 <= r2 && r1 <= l2
}