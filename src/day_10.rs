use std::{fs, iter};
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input/day_10.txt").unwrap();
    let commands = input.split('\n').map(Command::from).collect_vec();

    let mut cycle = 1;
    let mut x = 1;

    let mut checks = vec![220, 180, 140, 100, 60, 20];

    let crt_w = 40;
    let mut crt_pixel = 0;

    let mut solution_1 = 0;
    let mut solution_2 = String::new();
    commands.iter().for_each(|c| {

        (0..c.cycles()).for_each(|_| {
            let signal = cycle as isize * x;

            if !checks.is_empty() && cycle as isize == *checks.last().unwrap() {
                checks.pop().unwrap();
                solution_1 += signal;
            }

            if crt_pixel%crt_w <= x +1 && crt_pixel%crt_w >= x -1{
                solution_2.push('█');
            } else {
                solution_2.push('_');
            }

            cycle += 1;
            crt_pixel += 1;

            if crt_pixel % crt_w == 0 {
                solution_2.push('\n');
            }
        });

        match c {
            Command::Noop => {},
            Command::AddX(add) => {
                x += add;
            }
        }
    });
    assert_eq!(solution_1, 15020);
    assert_eq!(solution_2.trim(),r#"
                                ████_████_█__█__██__█____███___██__███__
                                █____█____█__█_█__█_█____█__█_█__█_█__█_
                                ███__███__█__█_█____█____█__█_█__█_█__█_
                                █____█____█__█_█_██_█____███__████_███__
                                █____█____█__█_█__█_█____█____█__█_█____
                                ████_█_____██___███_████_█____█__█_█____
                                "#.replace(' ', "").trim());
}


#[derive(Debug)]
enum Command {
    Noop,
    AddX(isize)
}


impl From<&str> for Command {
    fn from(s: &str) -> Self {
        let words = s.split(' ').collect_vec();
        match (words[0], words.get(1)) {
            ("noop", None) => Command::Noop,
            ("addx", Some(v)) => Command::AddX(v.parse::<isize>().unwrap()),
            _ => unimplemented!()
        }
    }
}

impl Command {
    fn cycles(&self) -> usize {
        match self {
            Command::Noop => 1,
            Command::AddX(_) => 2,
        }
    }

}