use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input/day_05.txt").unwrap();
    let moves = input.split("\n\n").nth(1).unwrap().split('\n')
        .map(|line|{
            let words = line.split(' ').collect_vec();
            Move {
                from: words[3].parse().unwrap(),
                to: words[5].parse().unwrap(),
                count: words[1].parse().unwrap()
            }
        }).collect_vec();

    let mut start_stacks = vec![
        vec!['T', 'R','D','H','Q','N','P','B'],
        vec!['V','T','J','B','G','W'],
        vec!['Q', 'M', 'V', 'S', 'D', 'H', 'R', 'N',],
        vec!['C','M','N','Z','P'],
        vec!['B','Z','D'],
        vec!['Z','W','C','V'],
        vec!['S', 'L',  'Q',  'V',  'C',  'N',  'Z',  'G',],
        vec!['V','N','D','M','J','G','L'],
        vec!['G','C','Z','F','M','P','T'],
    ];
    start_stacks.iter_mut().for_each(|stack|stack.reverse());


    let mut stacks = start_stacks.clone();
    moves.iter().for_each(|m| {
        (0..m.count).for_each(|_| {
            let taken = stacks[m.from-1].pop().unwrap();
            stacks[m.to-1].push(taken);
        })
    });
    let solution_1 = stacks.iter().map(|s|s.last().unwrap()).join("");

    let mut stacks = start_stacks.clone();
    moves.iter().for_each(|m| {
        let mut temp = vec![];
        (0..m.count).for_each(|_| {
            let taken = stacks[m.from-1].pop().unwrap();
            temp.push(taken);
        });

        temp.reverse();

        temp.iter().for_each(|item| {
            stacks[m.to-1].push(*item);
        });
    });
    let solution_2 = stacks.iter().map(|s|s.last().unwrap()).join("");

    assert_eq!(solution_1, "ZBDRNPMVH");
    assert_eq!(solution_2, "WDLPFNNNB");
}

struct Move {
    from: usize,
    to: usize,
    count: usize
}