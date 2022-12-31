use std::fmt::{Display, Formatter};
use std::{fs, isize};
use itertools::Itertools;
use pathfinding::num_traits::ToPrimitive;


fn main() {
    let input = fs::read_to_string("input/day_25.txt").unwrap();
    let sum = input.lines().map(Snafu::from).map(isize::from).sum::<isize>();
    assert_eq!("122-2=200-0111--=200", Snafu::from(sum as usize).as_snafu())
}


fn snafu_to_value(c: char) -> isize {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => unimplemented!("{}", c)
    }
}

fn snafu_from_value(v: isize) -> char {
    match v {
        -2 => '=',
        -1 => '-',
        0 => '0',
        1 => '1',
        2 => '2',
        _ => unimplemented!("{}", v)
    }
}

struct Snafu {
    values: Vec<isize>,
}

impl Snafu {
    fn new() -> Self { Snafu { values: vec![0] } }

    fn add(&mut self, n: usize) {
        self.values[0] += n as isize;
        let mut position = 0;
        while self.values[position] > 2 {
            let quotient = (self.values[position] + 2) / 5;
            let rem = (self.values[position] + 2) % 5 - 2;
            self.values[position] = rem;
            position += 1;
            if self.values.len() - 1 < position {
                self.values.push(quotient);
            } else {
                self.values[position] += quotient;
            }
        }
    }

    fn as_snafu(&self) -> String {
        self.values.iter().rev().map(|v| snafu_from_value(*v)).collect()
    }
}

impl From<&str> for Snafu {
    fn from(s: &str) -> Self {
        let values = s.chars()
            .map(snafu_to_value)
            .rev()
            .collect_vec();
        Snafu { values }
    }
}

impl From<usize> for Snafu {
    fn from(value: usize) -> Self {
        let mut c = Self::new();
        c.add(value);
        c
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_snafu())
    }
}

impl From<Snafu> for isize {
    fn from(snafu: Snafu) -> Self {
        snafu.values.iter()
            .enumerate()
            .map(|(i, c)| {
                5_usize.pow(i.to_u32().unwrap()) as isize * c
            }).sum::<isize>()
    }
}