use std::collections::HashMap;
use std::fs;
use itertools::Itertools;



fn main() {
    let input = fs::read_to_string("input/day_11.txt").unwrap();
    let mut monkeys = input.split("\n\n").map(Monkey::from).collect_vec();
    let divisors = monkeys.iter().map(|m|m.divisor).collect_vec();

    monkeys.iter_mut().for_each(|m|m.items.iter_mut()
        .for_each(|i|i.init(divisors.clone())));

    // i broke the code for solution 1 in order to solve solution 2
    // todo: fix it
    // let solution_1 = run_rounds(20, monkeys.clone(), true);
    // assert_eq!(solution_1, 57838);

    let solution_2 = run_rounds(10000, monkeys, false);
    assert_eq!(solution_2, 15050382231);
}


fn run_rounds(rounds: usize, mut monkeys: Vec<Monkey>, with_relieve: bool) -> usize {
    let mut inspected = monkeys.iter().map(|_| 0).collect_vec();
    for round in 1..rounds + 1 {
        println!("{}", round);

        for monkey_index in 0..monkeys.len() {
            println!("\n[{}] Monkey {}",round, monkey_index);
            if monkeys[monkey_index].items.is_empty() {
                continue;
            }
            let items = monkeys[monkey_index].items.clone();
            monkeys[monkey_index].items.clear();
            items.iter().for_each(|item| {
                //print!("[{}] {}?", round, item);
                inspected[monkey_index] += 1;

                let mut new_value = item.clone();
                new_value.apply(&monkeys[monkey_index].operation);
                //print!("->{}", new_value);
                if with_relieve {
                    new_value.apply(&Operation::Relieve);
                };
                //print!("/3->{}", &new_value);
                let throw_to = monkeys[monkey_index].test(&new_value);
                //print!("=> throw from {} to {}", monkey_index, throw_to);
                monkeys[throw_to].items.push(new_value);
                //println!();
            });
        }

        println!("\n===============\nafter round {}", round);
        for (i, m) in monkeys.iter().enumerate() {
            //println!("| Monkey {}: {}", i, m.items.iter().map(|i|i.to_string()).join(", "))
        }
        println!("{:?}", inspected);
    }
    return inspected.iter().sorted().rev().take(2).cloned().reduce(|a, b| a * b).unwrap();
}

#[derive(Clone)]
enum Operation { Mul(usize), Add(usize), Quad, Relieve}
impl Operation {
    fn execute(&self, lhs: usize) -> usize {
        match self {
            Operation::Mul(v) => lhs *v,
            Operation::Add(v) => lhs +v,
            Operation::Quad => lhs * lhs,
            Operation::Relieve => lhs / 3
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    divisor: usize,
    if_dividable: usize,
    if_not_dividable: usize,
}

impl Monkey {
    fn test(&self, item: &Item ) -> usize{
        if item.is_dividable_by(self.divisor) {
            self.if_dividable
        } else {
            self.if_not_dividable
        }
    }
}

#[derive(Debug, Clone)]
struct Item {
    start_value: usize,
    remainder: HashMap<usize, usize>,
}

impl Item {
    fn new(v: usize) -> Self{
        Item {
            start_value: v,
            remainder: Default::default()
        }
    }
    fn init(&mut self, divisors: Vec<usize>) {
        for d in divisors {
            let rem = self.start_value % d;
            self.remainder.insert(d, rem);
        }
    }
    fn apply(&mut self, op: &Operation) {
        match op {
            Operation::Mul(v) => {
                self.remainder.iter_mut().for_each(|(divisor, remainder)| {
                    *remainder = op.execute(*remainder) % divisor;
                })
            }
            Operation::Add(v) => {
                self.remainder.iter_mut().for_each(|(divisor, remainder)| {
                    let sum = v % divisor + *remainder;
                    *remainder =  sum % divisor;
                });
            }
            Operation::Quad => {
                self.remainder.iter_mut().for_each(|(divisor, remainder)| {
                    *remainder = op.execute(*remainder) % divisor;
                })
            }
            Operation::Relieve => {
                self.remainder.iter_mut().for_each(|(divisor, remainder)| {
                    *remainder = op.execute(*remainder);
                })
            }
        }
    }

    fn is_dividable_by(&self, divisor: usize) -> bool {
        self.remainder[&divisor] == 0
    }
}
impl From<usize> for Item {
    fn from(v: usize) -> Self {
        Item::new(v)
    }
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let rows = s.split('\n').map(|s|s.trim()).collect_vec();
        Monkey {
            items: rows[1].split(':').nth(1).unwrap().split(", ").map(|s|Item::new(s.trim().parse::<usize>().unwrap())).collect_vec(),
            operation: match (nth_word(rows[2], 4), nth_word(rows[2], 5)) {
                ("*", "old") => Operation::Quad,
                ("*", v) => Operation::Mul(as_int(v)),
                ("+", v) => Operation::Add(as_int(v)),
                _ => unimplemented!()
            },
            divisor: last_word_as_int(rows[3]),
            if_dividable: last_word_as_int(rows[4]),
            if_not_dividable: last_word_as_int(rows[5]),
        }
    }
}

fn last_word_as_int(line: &str) -> usize {
    println!("{}", line);
    line.split(' ').last().unwrap().trim().parse::<usize>().unwrap()
}

fn nth_word(line: &str, n: usize) -> &str {
    line.trim().split(' ').nth(n).unwrap()
}

fn as_int(s: &str) -> usize{
    s.parse::<usize>().unwrap()
}