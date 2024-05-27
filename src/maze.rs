use serde::{Deserialize, Serialize};
use std::io;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LinkType {
    Path,
    Wall,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    Blank,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cell {
    pub top: LinkType,
    pub right: LinkType,
    pub bottom: LinkType,
    pub left: LinkType,
    pub direction: Direction,
    pub in_maze: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Maze {
    data: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
    start_point: Coords,
    end_point: Coords,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Coords {
    pub x: usize,
    pub y: usize,
}

pub trait MazeGenerator {
    fn generate(&self, maze: &mut Maze) -> Result<(), io::Error>;
}

impl Maze {
    pub fn get_directions_possible(&self, coords: &Coords) -> Vec<Direction> {
        let mut vec: Vec<Direction> = Vec::new();

        if coords.x > 0 {
            vec.push(Direction::Left);
        }

        if coords.y > 0 {
            vec.push(Direction::Up);
        }

        if coords.x < self.width - 1 {
            vec.push(Direction::Right);
        }

        if coords.y < self.height - 1 {
            vec.push(Direction::Down);
        }

        vec
    }

    pub fn print_to_console(&self) {
        let mut lines: Vec<Vec<char>> = vec![vec![]; self.height * 2 + 1];
        lines[0].extend(['+', '-', '-', '-'].iter().cycle().take(self.width * 4 + 1));
        for y in 0..self.height {
            let print_y = y * 2 + 1;
            lines[print_y].push('|');
            lines[print_y + 1].push('+');
            for x in 0..self.width {
                let right = match self.data[y][x].right {
                    LinkType::Path => ' ',
                    LinkType::Wall => '|',
                };

                let bottom = match self.data[y][x].bottom {
                    LinkType::Path => ' ',
                    LinkType::Wall => '-',
                };

                lines[print_y].extend([' ', ' ', ' ', right].iter());
                lines[print_y + 1].extend([bottom, bottom, bottom, '+'].iter());
            }
        }

        for line in lines {
            let line: String = line.iter().collect();
            eprintln!("{line}");
        }
    }

    pub fn borrow_cell(&self, coords: &Coords) -> &Cell {
        &self.data[coords.y][coords.x]
    }

    pub fn borrow_cell_mut(&mut self, coords: &Coords) -> &mut Cell {
        &mut self.data[coords.y][coords.x]
    }

    pub fn get_possible_moves(&self, coords: &Coords) -> Vec<Coords> {
        let mut ret_vec = Vec::new();

        if self.data[coords.y][coords.x].top == LinkType::Path {
            ret_vec.push(Coords {
                x: coords.x,
                y: coords.y - 1,
            });
        }

        if self.data[coords.y][coords.x].right == LinkType::Path {
            ret_vec.push(Coords {
                x: coords.x + 1,
                y: coords.y,
            });
        }

        if self.data[coords.y][coords.x].bottom == LinkType::Path {
            ret_vec.push(Coords {
                x: coords.x,
                y: coords.y + 1,
            });
        }

        if self.data[coords.y][coords.x].left == LinkType::Path {
            ret_vec.push(Coords {
                x: coords.x - 1,
                y: coords.y,
            });
        }

        ret_vec
    }

    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_start_point(&self) -> &Coords {
        &self.start_point
    }

    pub fn get_end_point(&self) -> &Coords {
        &self.end_point
    }
}

pub struct MazeBuilder {
    generator: Box<dyn MazeGenerator>,
}

impl MazeBuilder {
    pub fn from_generator(generator: Box<dyn MazeGenerator>) -> MazeBuilder {
        MazeBuilder {
            generator: generator,
        }
    }
    pub fn generate(&self, width: usize, height: usize) -> Result<Maze, io::Error> {
        let base_cell = Cell {
            top: LinkType::Wall,
            right: LinkType::Wall,
            bottom: LinkType::Wall,
            left: LinkType::Wall,
            in_maze: false,
            direction: Direction::Blank,
        };

        let mut maze = Maze {
            width: width,
            height: height,
            data: vec![vec![base_cell; width]; height],
            start_point: Coords { x: 0, y: 0 },
            end_point: Coords {
                x: width - 1,
                y: height - 1,
            },
        };

        self.generator.generate(&mut maze)?;
        Ok(maze)
    }
}
