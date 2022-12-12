use std::fmt::{Display, Formatter};
use std::fs;
use itertools::Itertools;
use ndarray::{Array2, ArrayBase, Ix, Ix2, OwnedRepr};
use pathfinding::prelude::astar;
use crate::Label::{Basic, End, Start};

fn main() {
    let input = fs::read_to_string("input/day_12.txt").unwrap();
    let rows = input.split('\n').collect_vec();
    let width = rows.first().unwrap().len();
    let height = rows.len();

    let chars = input.replace('\n', "").chars().map(Position::from).collect_vec();

    let grid = Array2::from_shape_vec((height, width), chars).unwrap();
    let start = grid.indexed_iter()
        .find(|(_, pos)| pos.is_start())
        .map(|(| coord, pos)| Node { coord, pos: pos.clone() })
        .unwrap();

    let goal = grid.indexed_iter()
        .find(|(_, pos)| pos.is_end())
        .map(|(| coord, pos)| Node { coord, pos: pos.clone() })
        .unwrap();

    let solution_1 = astar(&start,
                           |n| successors(n, &grid),
                           |n| heuristic(n, &goal),
                           |n| n.pos.is_end())
        .map(|(_, cost)| cost)
        .unwrap();


    let solution_2 = grid.indexed_iter().filter(|(_, pos)| {
        pos.elevation_as_int() == 0
    }).map(|(coord, pos)| Node {
        coord,
        pos: pos.clone(),
    }).filter_map(|start| {
        astar(&start,
              |n| successors(n, &grid),
              |n| heuristic(n, &goal),
              |n| n.pos.is_end())
            .map(|(_, cost)| cost)
    }).min().unwrap();

    assert_eq!(solution_1, 472);
    assert_eq!(solution_2, 465);
}

fn successors(n: &Node, grid: &ArrayBase<OwnedRepr<Position>, Ix2>) -> Vec<(Node, usize)> {
    let up = (n.coord.0 as isize + 1, n.coord.1 as isize);
    let down = (n.coord.0 as isize - 1, n.coord.1 as isize);
    let left = (n.coord.0 as isize, n.coord.1 as isize + 1);
    let right = (n.coord.0 as isize, n.coord.1 as isize - 1);

    let nodes = vec![up, down, left, right].iter()
        .map(|(x, y)| (*x as Ix, *y as Ix))
        .filter_map(|c| grid.get(c).map(|pos| {
            Node { coord: c, pos: pos.clone() }
        }))
        .filter(|succ|
            (succ.pos.elevation_as_int() as isize - n.pos.elevation_as_int() as isize) <= 1
        )
        .map(|n| (n, 1))
        .collect_vec();

    nodes
}

fn heuristic(a: &Node, b: &Node) -> usize {
    (((b.coord.0 as isize - a.coord.0 as isize).pow(2)
        + (b.coord.1 as isize - a.coord.1 as isize).pow(2)) as f64).sqrt() as usize
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    coord: (Ix, Ix),
    pos: Position,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Label { Start, End, Basic, }

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position {
    label: Label,
    elevation: char,
}

impl Position {
    fn is_start(&self) -> bool { matches!(self.label, Start) }
    fn is_end(&self) -> bool { matches!(self.label, End) }
}

impl From<char> for Position {
    fn from(s: char) -> Self {
        match s {
            'S' => Position { label: Start, elevation: 'a' },
            'E' => Position { label: End, elevation: 'z' },
            e => Position { label: Basic, elevation: e },
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.elevation) }
}


impl Position {
    fn elevation_as_int(&self) -> usize { self.elevation as usize - 97 }
}
