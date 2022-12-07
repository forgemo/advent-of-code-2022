use std::collections::HashMap;
use std::fs;
use itertools::Itertools;
use crate::Command::{DirNode, FileNode, IntoDir, List, JumpToRoot, MoveUp};

fn main() {
    let input = fs::read_to_string("input/day_07.txt").unwrap();
    let commands = input.split('\n')
        .map(Command::from)
        .collect_vec();

    let mut path =  vec![];
    let mut dirs: Vec<String> = vec![];
    let mut map: HashMap<String, usize> = HashMap::new();

    commands.iter().for_each(|c|{
        match c {
            JumpToRoot => { path = vec![]; }
            MoveUp => { path.pop(); },
            IntoDir(name) => path.push(name.to_string()),
            List => {},
            FileNode(size, _) => {
                map.entry(path.join("/"))
                    .and_modify(|value| *value+=size)
                    .or_insert(*size);
            },
            DirNode(name) => {
                dirs.push(format!("{}/{}", path.join("/"), name))
            }
        };
    });


    let dir_sizes = dirs.iter().map(|d| {
        let sum = map.iter()
            .filter(|(a, _b)| a.starts_with(d))
            .map(|(_, b)|b)
            .sum::<usize>();
        sum
    }).collect_vec();


    // solution 1

    let solution_1 = dir_sizes.iter()
        .filter(|size| **size <= 100000)
        .sum::<usize>();


    // solution 2

    let total_space = 70000000;
    let min_needed_space = 30000000;
    let total_allocated_space = map.iter()
        .map(|(_, b)|b)
        .sum::<usize>();

    let current_free_space = total_space - total_allocated_space;
    let needed_space = min_needed_space - current_free_space;

    let solution_2 = dir_sizes.iter()
        .filter(|size| **size >= needed_space)
        .min().unwrap();

    assert_eq!(solution_1, 1206825);
    assert_eq!(solution_2, &9608311);
}


#[derive(Debug, Clone)]
enum Command {
    JumpToRoot,
    MoveUp,
    IntoDir(String),
    List,
    FileNode(usize, String),
    DirNode(String)
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        let words = s.split(' ').collect_vec();
        let command = match (words[0], words[1], words.get(2)) {
            ("$", "cd", Some(&"/")) => JumpToRoot,
            ("$", "cd", Some(&"..")) => MoveUp,
            ("$", "cd", Some(name)) => IntoDir(name.to_string()),
            ("$", "ls", _) => List,
            ("dir", name, _) => DirNode(name.to_string()),
            (size, name, _) => FileNode(size.parse::<usize>().unwrap(), name.to_string())
        };
        command
    }
}