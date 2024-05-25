use clap::Parser;
use maze_image_builder::ConfigArray;

mod maze;
mod maze_image_builder;

#[derive(Parser, Debug)]
pub struct CommandArgs {
    ///Width of maze
    pub width: usize,

    ///height of maze
    pub height: usize,

    ///path of output png
    #[arg(long, default_value = "./out.png")]
    pub path_out: String,

    #[arg(long, action)]
    pub console_print: bool,

    #[arg(long, default_value = "10")]
    pub cell_width: u32,

    #[arg(long, default_value = "10")]
    pub cell_height: u32,
}

pub fn main_run() {
    let config = CommandArgs::parse();
    let mut maze = maze::Maze::new(config.width, config.height);
    if let Err(e) = maze.generate() {
        panic!("Error : {}", e.to_string());
    }
    if config.console_print {
        maze.print_to_console();
    }

    let config_array = ConfigArray {
        cell_width: config.cell_width,
        cell_height: config.cell_height,
    };

    let mut image_builder = maze_image_builder::MazeImageBuilder::new(config_array, &maze);
    let image = image_builder.build_image();
    image.save(config.path_out).expect("Can't save file");
}
