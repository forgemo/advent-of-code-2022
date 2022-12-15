use std::cmp::Ordering;
use std::fmt::{Display, Formatter, write};
use std::fs;
use itertools::{EitherOrBoth, Itertools};
use ndarray::stack;
use crate::Value::{Integer, List};


// this solution hat not been "cleaned up" and shows the dirty truth
fn main() {
    let input = fs::read_to_string("input/day_13.txt").unwrap();
    //let input = fs::read_to_string("input/day_13_sample.txt").unwrap();

   let mut parsed = input.split("\n")
        .filter(|s|!s.trim().is_empty())
        .map(Value::from)
        .collect_vec();

    let solution_1 = parsed.iter()
        .tuples::<(_, _)>()
        .enumerate()
        .map(|(i, (l, r))| {
            println!("== Pair {} ==", i+1);
            (i+1, l.compare(r))
        })
        .filter(|(i, o)| matches!(o, Order::Valid))
        .map(|(i, _)| i)
        .sum::<usize>();


    let marker_1 = List(vec![List(vec![Integer(2)])]);
    let marker_2 = List(vec![List(vec![Integer(6)])]);
    parsed.push(marker_1.clone());
    parsed.push(marker_2.clone());

    parsed.sort_by(|a, b| {
       match a.compare(&b) {
           Order::Unknown => Ordering::Equal,
           Order::Valid => Ordering::Less,
           Order::Wrong => Ordering::Equal
       }
    });
    let solution_2 = parsed.iter().enumerate()
        .filter(|(_, v)| {
            v == &&marker_1 || v == &&marker_2
        })
        .map(|(i, p)| {
            i +1
        }).reduce(|a, b| a * b).unwrap();



    println!("{}", parsed.iter().map(|v| format!("{}", v)).join("\n"));
    println!("\n\n\n {}", solution_2);
    assert_eq!(solution_1, 5252);
}


#[derive(Debug, Clone, Eq, PartialEq)]
enum Value {
    List(Vec<Value>),
    Integer(usize)
}

impl Value {
    fn to_list(&self) -> Value {
        match self {
            List(v) => Value::List(v.clone()),
            Integer(v) => Value::List(vec![Integer(*v)])
        }
    }

    fn compare(&self, rhs: &Value) -> Order {
        println!("Compare {} vs {}", self, rhs);

        let o = match (self, rhs) {
            (Integer(l), Integer(r)) => match (l,r) {
                (l,r ) if l < r => Order::Valid,
                (l,r ) if l > r => Order::Wrong,
                (l,r ) if l == r => Order::Unknown,
                _ => unreachable!()
            },
            (Integer(_), List(_)) => self.to_list().compare(rhs),
            (List(_), Integer(_)) => self.compare(&rhs.to_list()),
            (List(l), List(r)) => {
                let pairs = l.iter()
                    .zip_longest(r.iter());

                let mut is_valid = false;
                for pair in pairs {
                    match pair {
                        EitherOrBoth::Both(l, r) => {
                            let o = l.compare(r);
                            match o {
                                Order::Unknown => {}
                                Order::Valid => return o,
                                Order::Wrong => return o
                            }
                        }
                        EitherOrBoth::Right(_) => {
                            is_valid = true;
                        }
                        EitherOrBoth::Left(_) => {
                            if !is_valid {
                                return Order::Wrong
                            }
                        }
                    }
                }
                if is_valid {
                    Order::Valid
                } else {
                    Order::Unknown
                }
            },
        };
        println!("{:?}", o);
        o
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            List(v) => write!(f, "[{}]", v.iter().map(|v| format!("{}", v)).join(",")),
            Value::Integer(v) => write!(f, "{}", v),
        }
    }
}

impl From<&str> for Value {
    fn from(line: &str) -> Self {

        let mut digit_buf = String::new();
        let mut open_stack = vec![];
        for c in line.trim().chars() {
            match c {
                '[' => {
                    let new = List(vec![]);
                    open_stack.push(new)
                },
                ']' => {
                    if !digit_buf.is_empty() {
                        let num = digit_buf.parse::<usize>().unwrap();
                        if let Some(List(list)) = open_stack.last_mut() {
                            list.push(Value::Integer(num));
                        } else {
                            unimplemented!()
                        }
                        digit_buf.clear();
                    }

                    // --------
                    let closed_list = open_stack.pop().unwrap();
                    if let Some(List(list)) = open_stack.last_mut() {
                        list.push(closed_list);
                    } else {
                        return closed_list
                    }

                },
                ',' => {
                    if !digit_buf.is_empty() {
                        let num = digit_buf.parse::<usize>().unwrap();
                        if let Some(List(list)) = open_stack.last_mut() {
                            list.push(Value::Integer(num));
                        } else {
                            unimplemented!()
                        }
                        digit_buf.clear();
                    }
                },
                digit => digit_buf.push(digit),
            }
        };
        unimplemented!();
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Order {
    Unknown,
    Valid,
    Wrong
}


