use clap::Parser;

#[derive(Parser, Debug)]
pub struct CommandArgs {
    ///Width of maze
    pub width : usize,

    ///height of maze
    pub height : usize,

    ///path of output png
    #[arg(long, default_value="./out.png")]
    pub path_out : Option<String>
}

#[derive(Clone, Debug)]
enum LinkType {
    Path,
    Wall
}

#[derive(Clone, Debug)]
struct Cell {
    top: LinkType,
    right: LinkType,
    bottom : LinkType,
    left : LinkType,
    in_maze : bool
}

#[derive(Debug)]
pub struct Maze {
    data : Vec<Vec<Cell>>,
    width : usize,
    height : usize
}

impl Maze {
    pub fn new(width : usize, height : usize) -> Maze{
        Maze {
            width: width,
            height : height,
            data : vec![vec![Cell {top : LinkType::Wall, right : LinkType::Wall, bottom : LinkType::Wall, left: LinkType::Wall, in_maze: false};width]; height]
        }
    }

    pub fn generate(&mut self) {
        
    }
}

