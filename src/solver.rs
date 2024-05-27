mod Solver {
    use crate::maze::{Coords, Maze};

    pub struct Path {
        data: Vec<Coords>,
    }

    pub fn solve_maze(maze: &Maze) -> Path {
        let mut path = Path { data: Vec::new() };
    }
}
