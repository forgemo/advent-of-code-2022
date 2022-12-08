use std::fs;
use itertools::Itertools;
use ndarray::{Array2, Axis};

fn main() {
    let input = fs::read_to_string("input/day_08.txt").unwrap();

    let lines = input.split('\n').collect_vec();
    let cells = lines.iter().flat_map(|l|l.chars().map(|c|c.to_string().parse::<usize>().unwrap())).collect_vec();
    let grid = Array2::from_shape_vec((lines[0].len(), lines.len()), cells).unwrap();

    let solution_1= grid.indexed_iter().map(|((y, x), height)|{
        let (left, right) = grid.row(y).split_at(Axis(0),x);
        let (top, bottom) = grid.column(x).split_at(Axis(0),y);

        let from_left = left.iter().max().map(|m|m < height).unwrap_or(true);
        let from_right = right.iter().skip(1).max().map(|m|m < height).unwrap_or(true);
        let from_top = top.iter().max().map(|m|m < height).unwrap_or(true);
        let from_bottom = bottom.iter().skip(1).max().map(|m|m < height).unwrap_or(true);

        usize::from(from_left || from_right || from_top || from_bottom)
    }).sum::<usize>();

    let solution_2= grid.indexed_iter().map(|((y, x), height)|{
        let (left, right) = grid.row(y).split_at(Axis(0),x);
        let (top, bottom) = grid.column(x).split_at(Axis(0),y);

        let view_left = left.iter().rev().enumerate().find(|(_, h)| *h >= height).map(|(i, _)| i+1).unwrap_or(left.len());
        let view_right = right.iter().skip(1).enumerate().find(|(_, h)| *h >= height).map(|(i, _)| i+1).unwrap_or(right.len() - 1);
        let view_top = top.iter().rev().enumerate().find(|(_, h)| *h >= height).map(|(i, _)| i+1).unwrap_or(top.len());
        let view_bottom = bottom.iter().skip(1).enumerate().find(|(_, h)| *h >= height).map(|(i, _)| i+1).unwrap_or(bottom.len() - 1);

        view_left * view_right * view_bottom * view_top
    }).max().unwrap();

    assert_eq!(solution_1, 1676);
    assert_eq!(solution_2, 313200);
}
