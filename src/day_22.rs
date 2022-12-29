use std::collections::{BTreeMap};
use std::fmt::{Display, Formatter};
use std::fs;
use itertools::Itertools;
use crate::Direction::{Down, Left, Right, Up};
use crate::Instruction::*;
use crate::TileType::{Open, Wall};

fn main() {
    let input = fs::read_to_string("input/day_22.txt").unwrap();
    let mut world = World::from(input.as_str());
    world.execute_instructions();
    assert_eq!(world.solution_score(), 66292);

    world.reset_start_position();
    world.grid.fold_to_cube();
    world.execute_instructions();
    assert_eq!(world.solution_score(), 127012);
}


type TileId = Position;

enum TileType { Open, Wall }


#[derive(Clone, Debug)]
enum Instruction { Move(usize), Turn(TurnDirection) }

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Direction { Left, Right, Up, Down }

#[derive(Clone, Debug)]
enum TurnDirection { Left, Right }


#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Position {
    row: usize,
    tile: usize,
}


#[derive(Clone, Debug)]
struct Link {
    to: TileId,
    rotation: usize
}

struct LinkedTile {
    position: Position,
    links: BTreeMap<Direction, Link>,
    tile_type: TileType,
}


struct WrappingGrid {
    cube_size: usize,
    tiles: BTreeMap<TileId, LinkedTile>,
}

struct World {
    grid: WrappingGrid,
    direction: Direction,
    position: TileId,
    instructions: Vec<Instruction>,
}


impl World {
    fn from(input: &str) -> Self {
        let grid = WrappingGrid::from(input);
        let position = grid.tiles.values().min_by_key(|t| &t.position).unwrap().position.clone();
        let instructions = parse_instructions(input.split("\n\n").nth(1).unwrap());

        let start_dir = Right;
        Self {
            grid,
            direction: start_dir,
            position,
            instructions,
        }
    }

    fn reset_start_position(&mut self) {
        let position = self.grid.tiles.values().min_by_key(|t| &t.position).unwrap().position.clone();
        self.position = position;
    }

    fn execute_instructions(&mut self) {
        let instructions = self.instructions.clone();
        instructions.iter().for_each(|i| self.execute(i));
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Move(steps) => self.move_steps(*steps),
            Turn(direction) => self.direction = self.direction.turn(direction)
        }
    }

    fn solution_score(&self) -> usize {
        1000 * (self.position.row + 1) + 4 * (self.position.tile + 1) + self.direction.value()
    }


    fn move_steps(&mut self, count: usize) {
        for _ in 0..count {
            let new_pos = &self.grid.tiles[&self.position].links[&self.direction];
            if matches!(self.grid.tiles[&new_pos.to].tile_type, Wall) {
                break;
            }
            self.position = new_pos.to.clone();
            if new_pos.rotation % 4 > 0 {
                (0..new_pos.rotation).for_each(|_|self.execute(&Turn(TurnDirection::Right)));
            }
        }
    }
}



impl WrappingGrid {
    fn fold_to_cube(&mut self) {

        let seams = cube_seam_pairs(self.cube_size);

        seams.into_iter().for_each(|(rotation, a, b)| {
            let (a_direction, (a_from, a_to)) = a;
            let (b_direction, (b_from, b_to)) = b;

            let pairs =
                self.tiles_along_seam(&a_from, &a_to).into_iter()
                    .zip_eq(self.tiles_along_seam(&b_from, &b_to).into_iter())
                    .collect_vec();

            pairs.into_iter().for_each(|(a,b)| {
               self.tiles.get_mut(&a).unwrap().links.insert(a_direction.clone(), Link{to: b.clone(), rotation});
               self.tiles.get_mut(&b).unwrap().links.insert(b_direction.clone(), Link{to: a, rotation: 4-rotation});
            });


        });

    }

    fn tiles_along_seam(&self, from: &Position, to_excl: &Position) -> Vec<TileId> {
        let mut positions = vec![];
        let mut current = from.clone();
        while &current != to_excl {
            assert!(self.tiles.contains_key(&current), "invalid tile id {}", current);
            positions.push(current.clone());
            current = current.step_towards(to_excl);
        }
        positions
    }

}

impl Position {

    fn new(row: usize, tile: usize) -> Self {
        Position {row, tile}
    }

    fn neighbour(&self, direction: &Direction, bounds: (usize, usize, usize, usize)) -> Self {
        let (left_bound, right_bound, top_bound, bottom_bound) = bounds;
        match direction {
            Left => self.left(left_bound, right_bound),
            Right => self.right(left_bound, right_bound),
            Up => self.top(top_bound, bottom_bound),
            Down => self.bottom(top_bound, bottom_bound),
        }
    }

    fn left(&self, left_bound: usize, right_bound: usize) -> Self {
        Position {
            tile: if self.tile == left_bound { right_bound } else { self.tile - 1 }
            ,
            ..self.clone()
        }
    }
    fn right(&self, left_bound: usize, right_bound: usize) -> Self {
        Position {
            tile: if self.tile == right_bound { left_bound } else { self.tile + 1 }
            ,
            ..self.clone()
        }
    }
    fn top(&self, top_bound: usize, bottom_bound: usize) -> Self {
        Position {
            row: if self.row == top_bound { bottom_bound } else { self.row - 1 }
            ,
            ..self.clone()
        }
    }
    fn bottom(&self, top_bound: usize, bottom_bound: usize) -> Self {
        Position {
            row: if self.row == bottom_bound { top_bound } else { self.row + 1 }
            ,
            ..self.clone()
        }
    }
    fn step_towards(&self, other: &Position) -> Self {
        Position {
            row : (self.row as isize + (other.row as isize - self.row as isize).signum()) as usize,
            tile : (self.tile as isize + (other.tile as isize - self.tile as isize).signum()) as usize,
        }
    }
}

impl From<&char> for TileType {
    fn from(value: &char) -> Self {
        match value {
            '.' => Open,
            '#' => Wall,
            _ => unimplemented!("tile '{}' unexpected", value)
        }
    }
}

fn boundaries_for_tile(char_grid: &[Vec<char>], pos: &Position) -> (usize, usize, usize, usize) {
    let mut iter = char_grid[pos.row].iter().enumerate().filter(|(_, c)| c != &&' ').map(|(idx, _)| idx);
    let left = iter.next().unwrap();
    let right = iter.last().unwrap();
    let mut iter = char_grid.iter().enumerate().filter(|(_, row)| row[pos.tile] != ' ').map(|(idx, _)| idx);
    let top = iter.next().unwrap();
    let bottom = iter.last().unwrap();
    (left, right, top, bottom)
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    line.chars().group_by(|c| c.is_ascii_digit()).into_iter()
        .flat_map(|(is_digits, group)| {
            if is_digits {
                vec![Move(group.collect::<String>().parse::<usize>().unwrap())]
            } else {
                group.map(|char| match char {
                    'L' => Turn(TurnDirection::Left),
                    'R' => Turn(TurnDirection::Right),
                    _ => unimplemented!("unexpected direction {}", char)
                }).collect_vec()
            }
        }).collect()
}



type CubeSeam = (Direction, (Position, Position));

fn cube_seam_pairs(size: usize) -> Vec<(usize, CubeSeam, CubeSeam)> {
    let a_1 = (Position::new(0, size), Position::new(0, 2*size));
    let a_2 = (Position::new(3*size, 0), Position::new(4*size, 0));

    let b_1 = (Position::new(0, 2*size), Position::new(0, 3*size));
    let b_2 = (Position::new(4*size-1, 0), Position::new(4*size-1, size));

    let c_1 = (Position::new(3*size-1, size), Position::new(3*size-1, 2*size));
    let c_2 = (Position::new(3*size, size-1), Position::new(4*size, size-1));

    let d_1 = (Position::new(0, 3*size-1), Position::new(size, 3*size-1));
    let d_2 = (Position::new(3*size-1, 2*size-1), Position::new(2*size-1, 2*size-1));

    let e_1 = (Position::new(size-1, 2*size), Position::new(size-1, 3*size));
    let e_2 = (Position::new(size, 2*size-1), Position::new(2*size, 2*size-1));

    let f_1 = (Position::new(size, size), Position::new(2*size, size));
    let f_2 = (Position::new(2*size, 0), Position::new(2*size, size));

    let g_1 = (Position::new(0, size), Position::new(size, size));
    let g_2 = (Position::new(3*size-1, 0), Position::new(2*size-1, 0));

    vec![
        (1, (Up, a_1), (Left, a_2)),
        (0, (Up, b_1), (Down, b_2)),
        (1, (Down, c_1), (Right, c_2)),
        (2, (Right, d_1), (Right, d_2)),
        (1, (Down, e_1), (Right, e_2)),
        (3, (Left, f_1), (Up, f_2)),
        (2, (Left, g_1), (Left, g_2)),
    ]
}


impl Direction {
    fn turn(&self, direction: &TurnDirection) -> Direction {
        match (self, direction) {
            (Left, TurnDirection::Left) => Down,
            (Left, TurnDirection::Right) => Up,
            (Right, TurnDirection::Left) => Up,
            (Right, TurnDirection::Right) => Down,
            (Up, TurnDirection::Left) => Left,
            (Up, TurnDirection::Right) => Right,
            (Down, TurnDirection::Left) => Right,
            (Down, TurnDirection::Right) => Left,
        }
    }

    fn value(&self) -> usize {
        match self {
            Right => 0,
            Down => 1,
            Left => 2,
            Up => 3,
        }
    }

    fn all() -> Vec<Self>{
        vec![Left, Right, Up, Down]
    }
}



impl From<&str> for WrappingGrid {
    fn from(input: &str) -> Self {
        let lines = input.lines().take_while(|l| !l.is_empty()).collect_vec();
        let max_width = lines.iter().map(|l| l.len()).max().unwrap();
        let cube_size = lines.iter().map(|l|l.chars().filter(|c|*c!=' ').count()).min().unwrap();
        let char_grid = lines.iter().map(|l| l.chars().pad_using(max_width, |_| ' ').collect_vec()).collect_vec();
        let tiles = char_grid.iter()
            .enumerate()
            .flat_map(|(row_index, row)| {
                row.iter().enumerate()
                    .filter(|(_, tile_char)| **tile_char != ' ')
                    .map(|(tile_index, tile_char)| {
                        let pos = Position { row: row_index, tile: tile_index };
                        let bounds = boundaries_for_tile(&char_grid, &pos);
                        LinkedTile {
                            links: Direction::all().into_iter().map(|d|{
                                let link = Link{to: pos.neighbour(&d, bounds), rotation: 0};
                                (d, link)
                            }).collect(),
                            position: pos,
                            tile_type: TileType::from(tile_char),
                        }
                    })
                    .map(|tile| (tile.position.clone(), tile))
                    .collect_vec()
            }).collect::<BTreeMap<TileId, LinkedTile>>();

        WrappingGrid {
            cube_size,
            tiles,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.row, self.tile)
    }
}