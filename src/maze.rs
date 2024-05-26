use std::io;
use std::marker::PhantomData;

#[derive(Clone, Debug, PartialEq)]
pub enum LinkType {
    Path,
    Wall,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
    Blank,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub top: LinkType,
    pub right: LinkType,
    pub bottom: LinkType,
    pub left: LinkType,
    pub direction: Direction,
    pub in_maze: bool,
}

#[derive(Debug)]
pub struct Maze {
    data: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
}

pub trait MazeGenerator {
    fn generate(maze: &mut Maze) -> Result<(), io::Error>;
}

impl Maze {
    pub fn get_directions_possible(&self, x: usize, y: usize) -> Vec<Direction> {
        let mut vec: Vec<Direction> = Vec::new();

        if x > 0 {
            vec.push(Direction::Left);
        }

        if y > 0 {
            vec.push(Direction::Up);
        }

        if x < self.width - 1 {
            vec.push(Direction::Right);
        }

        if y < self.height - 1 {
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

    pub fn borrow_cell(&self, x: usize, y: usize) -> &Cell {
        &self.data[y][x]
    }

    pub fn borrow_cell_mut(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.data[y][x]
    }
}

pub struct MazeBuilder<T> {
    generator_type: PhantomData<T>,
}

impl<T> MazeBuilder<T>
where
    T: MazeGenerator,
{
    pub fn generate(width: usize, height: usize) -> Result<Maze, io::Error> {
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
        };

        T::generate(&mut maze)?;
        Ok(maze)
    }
}
