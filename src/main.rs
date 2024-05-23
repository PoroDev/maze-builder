use clap::Parser;
use maze_builder::{CommandArgs, Maze};

fn main() {
    let args = CommandArgs::parse();
    let maze = Maze::new(args.width, args.height);
    println!("{maze:?}");
}
