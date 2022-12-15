use std::collections::HashSet;
use std::fmt::{Display, format, Formatter};
use std::fs;
use itertools::Itertools;


// this solution hat not been "cleaned up" and shows the dirty truth

fn main() {
    let input = fs::read_to_string("input/day_15.txt").unwrap();
    let puzzle_row = 2000000;
    let puzzle_range = 4000000;

    //let input = fs::read_to_string("input/day_15_sample.txt").unwrap();
    //let puzzle_row = 10;
    //let puzzle_range = 20;

    let sensors = input.split('\n').map(Sensor::from).collect_vec();

    let relevant_sensors = sensors.iter().collect_vec();

    let mut empty = empty_space_at_row(&relevant_sensors, puzzle_row);

    //println!("{}", relevant_sensors.iter().map(|s|format!("{}", s)).join(","));
    //empty.iter().sorted().for_each(|c|println!("{:?}", c));
    //println!("{} emtpy", empty.len());//,


    println!("starting with solution 2");

    let mut excluded = HashSet::new();
    for s in &relevant_sensors {
        excluded.insert(s.beacon);
        excluded.insert(s.coord);
    }


    let row =(0..puzzle_range+1)
        .inspect(|i| {
            if i% 10000 ==0 {
                println!("row {}" , i);
            }
        })
        .find(|i|!is_row_covered(&relevant_sensors, *i, puzzle_range)).unwrap();

    println!("solution 2 row {}", row);

    let col =(0..puzzle_range+1)
        .inspect(|i| {
            if i% 10000 ==0 {
                println!("col {}" , i);
            }
        })
        .find(|i|!is_col_covered(&relevant_sensors, *i, puzzle_range)).unwrap();

    println!("solution 2 col {}", col);

    println!("solution 2 {}", col * 4000000 + row);

    let solution_1 = empty.len();
    assert_eq!(solution_1, 5073496);
}



fn empty_space_at_row(sensors: &[&Sensor], row: isize) -> HashSet<(isize, isize)> {

    let mut empty = sensors.iter().flat_map(|s|s.empty_space_at_row(row))
        .collect::<HashSet<_>>();

    sensors.iter().for_each(|s|{
        //todo: needed?
        empty.remove(&s.coord);
    });
    empty
}

fn is_row_covered(sensors: &[&Sensor], row: isize, limit: isize) -> bool {
    //println!("check [{}]", row);
    sensors.iter()
        .filter_map(|s|s.covered_range_at_row(row))
        .map(Some)
        .sorted()
        //.inspect(|a| print!("{:?}, ", a))
        .reduce(|a,b| {
            //print!("\n reducing {:?} and {:?}", a, b);
            let result = match (a, b) {
                (None, _) => None,
                (_, None) => None,
                (Some(a), Some(b)) if b.0 > a.1 && a.1.abs_diff(b.0) > 1 => None,
                (Some(a), Some(b)) => Some((a.0.min(b.0), a.1.max(b.1)))
            };
            //println!(" -> {:?}", result);
            result
        })
        .flatten().map(|r|r.0 <= 0 && r.1 >= limit)
        .unwrap_or(false)
}

fn is_col_covered(sensors: &[&Sensor], col: isize, limit: isize) -> bool {
    //println!("check [{}]", row);
    sensors.iter()
        .filter_map(|s|s.covered_range_at_col(col))
        .map(Some)
        .sorted()
        //.inspect(|a| print!("{:?}, ", a))
        .reduce(|a,b| {
            //print!("\n reducing {:?} and {:?}", a, b);
            let result = match (a, b) {
                (None, _) => None,
                (_, None) => None,
                (Some(a), Some(b)) if b.0 > a.1 && a.1.abs_diff(b.0) > 1 => None,
                (Some(a), Some(b)) => Some((a.0.min(b.0), a.1.max(b.1)))
            };
            //println!(" -> {:?}", result);
            result
        })
        .flatten().map(|r|r.0 <= 0 && r.1 >= limit)
        .unwrap_or(false)
}


#[derive(Debug)]
struct Sensor {
    coord: (isize, isize),
    beacon: (isize, isize)
}

impl From<&str> for Sensor {
    fn from(s: &str) -> Self {
        let parts = s.split(':').collect_vec();
        let x = parts[0].split(' ').nth(2).unwrap().split('=').nth(1).unwrap().replace(',', "").parse::<isize>().unwrap();
        let y = parts[0].split(' ').nth(3).unwrap().split('=').nth(1).unwrap().parse::<isize>().unwrap();
        let tx = parts[1].trim().split(' ').nth(4).unwrap().split('=').nth(1).unwrap().replace(',', "").parse::<isize>().unwrap();
        let ty = parts[1].trim().split(' ').nth(5).unwrap().split('=').nth(1).unwrap().parse::<isize>().unwrap();
        Sensor {coord: (x,y), beacon: (tx, ty)}
    }
}

impl Sensor {


    fn covered_range_at_row(&self, row: isize) -> Option<(isize, isize)> {
        let total_dist = self.dist_to_beacon();
        let remaining_x_dist = total_dist - (row - self.coord.1).abs();
        if remaining_x_dist <= 0 {
                None
        } else {
            let a = self.coord.0 -remaining_x_dist;
            let b = self.coord.0 + remaining_x_dist;
            Some((a.min(b), a.max(b)))
        }
    }

    fn covered_range_at_col(&self, col: isize) -> Option<(isize, isize)> {
        let total_dist = self.dist_to_beacon();
        let remaining_y_dist = total_dist - (col - self.coord.0).abs();
        if remaining_y_dist <= 0 {
            None
        } else {
            let a = self.coord.1 - remaining_y_dist;
            let b = self.coord.1 + remaining_y_dist;
            Some((a.min(b), a.max(b)))
        }
    }

    fn empty_space_at_row(&self, row: isize) -> Vec<(isize, isize)> {
        let total_dist = self.dist_to_beacon();
        let remaining_x_dist = total_dist - (row - self.coord.1).abs();

        if remaining_x_dist <= 0 {
            return vec![]
        }
        (-remaining_x_dist..remaining_x_dist+1)
            .map(|i| (self.coord.0 + i, row))
            .filter(|coord| coord != &self.beacon)
            .filter(|coord| coord != &self.coord)
           .collect_vec()
    }

    fn dist_to_beacon(&self) -> isize {
        let (dist_x, dist_y) = self.vec_to_beacon();
        dist_x.abs() + dist_y.abs()
    }

    fn vec_to_beacon(&self) -> (isize, isize) {
        (self.beacon.0 - self.coord.0, self.beacon.1 - self.coord.1)
    }
}

impl Display for Sensor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})->({},{})", self.coord.0, self.coord.1, self.beacon.0, self.beacon.1)
    }
}