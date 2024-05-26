use crate::maze::{Direction, LinkType, Maze, MazeGenerator};
use rand::seq::SliceRandom;
use std::io;

pub struct WilsonGenerator;

impl MazeGenerator for WilsonGenerator {
    fn generate(maze: &mut Maze) -> Result<(), io::Error> {
        let mut remaining = maze.width * maze.height - 1;
        maze.borrow_cell_mut(0, 0).in_maze = true;
        let mut curr_x: usize = 1;
        let mut curr_y: usize = 0;
        while remaining > 0 {
            while maze.borrow_cell(curr_x, curr_y).in_maze {
                if curr_x < maze.width - 1 {
                    curr_x += 1;
                } else {
                    curr_x = 0;
                    curr_y += 1;
                    if curr_y > maze.height - 1 {
                        return Err(io::Error::new(io::ErrorKind::Other, "Exceeded maze range"));
                    }
                }
            }
            WilsonGenerator::random_walk(maze, curr_x, curr_y)?;

            let count = WilsonGenerator::follow_path(maze, curr_x, curr_y)?;

            remaining -= count;
        }
        Ok(())
    }
}

impl WilsonGenerator {
    fn random_walk(maze: &mut Maze, start_x: usize, start_y: usize) -> Result<(), io::Error> {
        let mut current_x = start_x;
        let mut current_y = start_y;

        while !maze.borrow_cell(current_x, current_y).in_maze {
            let possible_directions = maze.get_directions_possible(current_x, current_y);
            let rand_dir = possible_directions.choose(&mut rand::thread_rng()).unwrap();

            maze.borrow_cell_mut(current_x, current_y).direction = rand_dir.clone();

            match rand_dir {
                Direction::Down => {
                    current_y += 1;
                }

                Direction::Up => {
                    current_y -= 1;
                }

                Direction::Left => {
                    current_x -= 1;
                }

                Direction::Right => {
                    current_x += 1;
                }

                Direction::Blank => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "No direction available",
                    ));
                }
            }
        }
        Ok(())
    }

    fn follow_path(maze: &mut Maze, start_x: usize, start_y: usize) -> Result<usize, io::Error> {
        let mut current_x = start_x;
        let mut current_y = start_y;
        let mut count: usize = 0;

        while !maze.borrow_cell(current_x, current_y).in_maze {
            match maze.borrow_cell(current_x, current_y).direction {
                Direction::Up => {
                    let cell = maze.borrow_cell_mut(current_x, current_y);
                    cell.in_maze = true;
                    cell.top = LinkType::Path;
                    current_y -= 1;
                    maze.borrow_cell_mut(current_x, current_y).bottom = LinkType::Path;
                }

                Direction::Down => {
                    let cell = maze.borrow_cell_mut(current_x, current_y);
                    cell.in_maze = true;
                    cell.bottom = LinkType::Path;
                    current_y += 1;
                    maze.borrow_cell_mut(current_x, current_y).top = LinkType::Path;
                }

                Direction::Left => {
                    let cell = maze.borrow_cell_mut(current_x, current_y);
                    cell.in_maze = true;
                    cell.left = LinkType::Path;
                    current_x -= 1;
                    maze.borrow_cell_mut(current_x, current_y).right = LinkType::Path;
                }

                Direction::Right => {
                    let cell = maze.borrow_cell_mut(current_x, current_y);
                    cell.in_maze = true;
                    cell.right = LinkType::Path;
                    current_x += 1;
                    maze.borrow_cell_mut(current_x, current_y).left = LinkType::Path;
                }

                Direction::Blank => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "No direction available",
                    ));
                }
            }
            count += 1;
        }

        Ok(count)
    }
}
