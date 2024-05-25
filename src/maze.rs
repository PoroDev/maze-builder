use rand::seq::SliceRandom;
use std::io;

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

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        let base_cell = Cell {
            top: LinkType::Wall,
            right: LinkType::Wall,
            bottom: LinkType::Wall,
            left: LinkType::Wall,
            in_maze: false,
            direction: Direction::Blank,
        };

        Maze {
            width: width,
            height: height,
            data: vec![vec![base_cell; width]; height],
        }
    }

    pub fn generate(&mut self) -> Result<(), io::Error> {
        let mut remaining = self.width * self.height - 1;
        self.data[0][0].in_maze = true;
        let mut curr_x: usize = 1;
        let mut curr_y: usize = 0;
        while remaining > 0 {
            while self.data[curr_y][curr_x].in_maze {
                if curr_x < self.width - 1 {
                    curr_x += 1;
                } else {
                    curr_x = 0;
                    curr_y += 1;
                    if curr_y > self.height - 1 {
                        return Err(io::Error::new(io::ErrorKind::Other, "Exceeded maze range"));
                    }
                }
            }
            self.random_walk(curr_x, curr_y)?;

            let count = self.follow_path(curr_x, curr_y)?;

            remaining -= count;
        }
        Ok(())
    }

    fn get_directions_possible(&self, x: usize, y: usize) -> Vec<Direction> {
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

    fn random_walk(&mut self, start_x: usize, start_y: usize) -> Result<(), io::Error> {
        let mut current_x = start_x;
        let mut current_y = start_y;

        while !self.data[current_y][current_x].in_maze {
            let possible_directions = self.get_directions_possible(current_x, current_y);
            let rand_dir = possible_directions.choose(&mut rand::thread_rng()).unwrap();

            self.data[current_y][current_x].direction = rand_dir.clone();

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

    fn follow_path(&mut self, start_x: usize, start_y: usize) -> Result<usize, io::Error> {
        let mut current_x = start_x;
        let mut current_y = start_y;
        let mut count: usize = 0;

        while !self.data[current_y][current_x].in_maze {
            match self.data[current_y][current_x].direction {
                Direction::Up => {
                    self.data[current_y][current_x].in_maze = true;
                    self.data[current_y][current_x].top = LinkType::Path;
                    current_y -= 1;
                    self.data[current_y][current_x].bottom = LinkType::Path;
                }

                Direction::Down => {
                    self.data[current_y][current_x].in_maze = true;
                    self.data[current_y][current_x].bottom = LinkType::Path;
                    current_y += 1;
                    self.data[current_y][current_x].top = LinkType::Path;
                }

                Direction::Left => {
                    self.data[current_y][current_x].in_maze = true;
                    self.data[current_y][current_x].left = LinkType::Path;
                    current_x -= 1;
                    self.data[current_y][current_x].right = LinkType::Path;
                }

                Direction::Right => {
                    self.data[current_y][current_x].in_maze = true;
                    self.data[current_y][current_x].right = LinkType::Path;
                    current_x += 1;
                    self.data[current_y][current_x].left = LinkType::Path;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get_directions() {
        let maze = Maze::new(4, 4);
        let dir1 = maze.get_directions_possible(0, 0);
        assert_eq!(dir1, vec![Direction::Right, Direction::Down]);

        let dir2 = maze.get_directions_possible(3, 3);
        assert_eq!(dir2, vec![Direction::Left, Direction::Up]);

        let dir3 = maze.get_directions_possible(1, 2);
        assert_eq!(
            dir3,
            vec![
                Direction::Left,
                Direction::Up,
                Direction::Right,
                Direction::Down
            ]
        );
    }

    #[test]
    pub fn test_follow_path() {
        let mut maze = Maze::new(4, 4);
        maze.data[1][1].direction = Direction::Right;
        maze.data[1][2].direction = Direction::Down;
        maze.data[2][1].direction = Direction::Up;
        maze.data[2][2].direction = Direction::Left;
        assert_eq!(maze.follow_path(1, 1).unwrap(), 4);
        assert!(
            maze.data[1][1].bottom == LinkType::Path && maze.data[1][1].right == LinkType::Path
        );
        assert!(maze.data[1][2].left == LinkType::Path && maze.data[1][2].bottom == LinkType::Path);
        assert!(maze.data[2][1].right == LinkType::Path && maze.data[2][1].top == LinkType::Path);
        assert!(maze.data[2][2].left == LinkType::Path && maze.data[2][2].top == LinkType::Path);
    }

    #[test]
    pub fn test_generate_without_error() {
        let mut maze = Maze::new(40, 40);
        assert!(maze.generate().is_ok());
        maze.print_to_console();
    }

    #[test]
    pub fn test_print_to_console() {
        let mut maze = Maze::new(4, 4);
        maze.data[1][1].right = LinkType::Path;
        maze.data[1][2].left = LinkType::Path;
        maze.print_to_console();
        assert!(true);
    }
}
