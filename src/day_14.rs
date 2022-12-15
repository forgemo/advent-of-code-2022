use std::collections::{HashMap, HashSet};
use std::fs;
use itertools::Itertools;
use ndarray::Array2;


// this solution hat not been "cleaned up" and shows the dirty truth
fn main() {
    let input = fs::read_to_string("input/day_14.txt").unwrap();
    //let input = fs::read_to_string("input/day_14_sample.txt").unwrap();
    let rock_traces = input.split('\n').map(|path| path.split(" -> ")
        .map(|points| points.split(',').map(|s|s.parse::<usize>().unwrap()).next_tuple::<(_, _)>().unwrap()).collect_vec()).collect_vec();

    let mut world = HashSet::new();

    let mut max_y = 0;
    rock_traces.iter().for_each(|trace|{
        trace.iter().tuple_windows::<(_,_)>().for_each(|(a, b)| {
            let x_diff = b.0 as isize - a.0 as isize;
            let y_diff = b.1 as isize - a.1 as isize;

            println!("insert {:?}", a);
            world.insert(*a);

            if x_diff != 0 {
                (0..x_diff.abs() + 1).for_each(|i| {
                    let rock =  ((a.0 as isize + i*x_diff.signum()) as usize, a.1);
                    world.insert(rock);
                    println!("insert {:?}",rock);
                })
            } else if y_diff != 0 {
                (0..y_diff.abs() +1).for_each(|i| {
                    let rock = (a.0, (a.1 as isize + i*y_diff.signum()) as usize);
                    max_y = max_y.max(rock.1);
                    world.insert(rock);
                    println!("insert {:?}", rock );
                })
            } else {
                unreachable!()
            }
        });

    });

    max_y += 2;
    println!("{:?}", world);

    let source = (500 as usize, 0 as usize);
    let mut count = 0;
    let mut sand_pos = source.clone();
    loop {
        println!("[{}] {:?}", count, sand_pos);
        if sand_pos.1 > 2000 {
            break
        }
        let down = (sand_pos.0, sand_pos.1 + 1);
        let down_left = (sand_pos.0 - 1, sand_pos.1 + 1);
        let down_right = (sand_pos.0 + 1, sand_pos.1 + 1);

        let free_spot = [down, down_left, down_right].into_iter()
            .filter(|(_, y)| *y < max_y)
            .find(|pos| !world.contains(pos));

        if let Some(free_pos) = free_spot {
            println!("[{}] free spot at {:?}", count, free_pos);
            sand_pos = free_pos;
        } else {
            if sand_pos == source {
                count += 1;
                break
            }
            println!("[{}] insert {:?}", count, sand_pos);
            world.insert(sand_pos);
            sand_pos = source;
            count += 1;
        }
    }

    println!("{:?}", rock_traces);
    println!("{:?}", world);
    println!("{}", count);
    println!("max  {}", max_y);
}

