use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use thiserror::Error;

use crate::maze::{Coords, Maze};

#[derive(Error, Debug)]
pub enum SolverError {
    #[error("End of maze was not reached by solver")]
    EndNotReached,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Path {
    pub data: Vec<Coords>,
}

pub fn solve_maze(maze: &Maze) -> Result<Path, SolverError> {
    let mut path = Path { data: Vec::new() };
    let start_coords = maze.get_start_point().clone();
    let end_coords = maze.get_end_point().clone();

    let mut map: HashMap<Coords, Coords> = HashMap::new();
    let mut queue: VecDeque<Coords> = VecDeque::new();
    queue.push_back(start_coords);

    while !queue.is_empty() {
        let coords = queue.pop_front().unwrap();
        let possible_moves = maze.get_possible_moves(&coords);
        for possible_move in possible_moves {
            if map.contains_key(&possible_move) {
                continue;
            }
            map.insert(possible_move, coords.clone());
            queue.push_back(possible_move);
        }
    }

    if !map.contains_key(&end_coords) {
        return Err(SolverError::EndNotReached);
    }

    let mut current_coords = &end_coords;
    while current_coords != &start_coords {
        path.data.push(current_coords.clone());
        current_coords = map.get(&current_coords).unwrap();
    }
    path.data.push(start_coords);
    path.data.reverse();
    Ok(path)
}

#[cfg(test)]
mod tests_solver {
    use crate::{maze::MazeBuilder, maze_generator::wilson::WilsonGenerator, solver::solve_maze};

    #[test]
    pub fn test_solve_maze() {
        let generator = Box::new(WilsonGenerator);
        let maze = MazeBuilder::from_generator(generator)
            .generate(10, 10)
            .unwrap();
        println!("{:?}", solve_maze(&maze));
    }
}
