use crate::maze::{Coords, Direction, LinkType, Maze, MazeGenerator};
use rand::seq::SliceRandom;
use std::io;

pub struct WilsonGenerator;

impl MazeGenerator for WilsonGenerator {
    fn generate(&self, maze: &mut Maze) -> Result<(), io::Error> {
        let (width, height) = maze.get_dimensions();
        let mut remaining = width * height - 1;
        maze.borrow_cell_mut(&Coords { x: 0, y: 0 }).in_maze = true;
        let mut curr_coords = Coords { x: 1, y: 0 };
        while remaining > 0 {
            while maze.borrow_cell(&curr_coords).in_maze {
                if curr_coords.x < width - 1 {
                    curr_coords.x += 1;
                } else {
                    curr_coords.x = 0;
                    curr_coords.y += 1;
                    if curr_coords.y > height - 1 {
                        return Err(io::Error::new(io::ErrorKind::Other, "Exceeded maze range"));
                    }
                }
            }
            WilsonGenerator::random_walk(maze, &curr_coords)?;

            let count = WilsonGenerator::follow_path(maze, &curr_coords)?;

            remaining -= count;
        }
        Ok(())
    }
}

impl WilsonGenerator {
    fn random_walk(maze: &mut Maze, start_coords: &Coords) -> Result<(), io::Error> {
        let mut current_coords = start_coords.clone();

        while !maze.borrow_cell(&current_coords).in_maze {
            let possible_directions = maze.get_directions_possible(&current_coords);
            let rand_dir = possible_directions.choose(&mut rand::thread_rng()).unwrap();

            maze.borrow_cell_mut(&current_coords).direction = rand_dir.clone();

            match rand_dir {
                Direction::Down => {
                    current_coords.y += 1;
                }

                Direction::Up => {
                    current_coords.y -= 1;
                }

                Direction::Left => {
                    current_coords.x -= 1;
                }

                Direction::Right => {
                    current_coords.x += 1;
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

    fn follow_path(maze: &mut Maze, coords: &Coords) -> Result<usize, io::Error> {
        let mut current_coords = coords.clone();
        let mut count: usize = 0;

        while !maze.borrow_cell(&current_coords).in_maze {
            match maze.borrow_cell(&current_coords).direction {
                Direction::Up => {
                    let cell = maze.borrow_cell_mut(&current_coords);
                    cell.in_maze = true;
                    cell.top = LinkType::Path;
                    current_coords.y -= 1;
                    maze.borrow_cell_mut(&current_coords).bottom = LinkType::Path;
                }

                Direction::Down => {
                    let cell = maze.borrow_cell_mut(&current_coords);
                    cell.in_maze = true;
                    cell.bottom = LinkType::Path;
                    current_coords.y += 1;
                    maze.borrow_cell_mut(&current_coords).top = LinkType::Path;
                }

                Direction::Left => {
                    let cell = maze.borrow_cell_mut(&current_coords);
                    cell.in_maze = true;
                    cell.left = LinkType::Path;
                    current_coords.x -= 1;
                    maze.borrow_cell_mut(&current_coords).right = LinkType::Path;
                }

                Direction::Right => {
                    let cell = maze.borrow_cell_mut(&current_coords);
                    cell.in_maze = true;
                    cell.right = LinkType::Path;
                    current_coords.x += 1;
                    maze.borrow_cell_mut(&current_coords).left = LinkType::Path;
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
