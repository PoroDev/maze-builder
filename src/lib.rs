use clap::{ArgGroup, Parser};
use maze::{MazeBuilder, MazeGenerator};
use maze_generator::wilson::WilsonGenerator;
use maze_image_builder::ConfigArray;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fs;
use std::process::exit;

mod image_drawer;
mod maze;
mod maze_generator;
mod maze_image_builder;
mod solver;

#[derive(Parser, Debug)]
#[command(name = "Maze Generator")]
#[command(group(ArgGroup::new("generator").required(false).args(&["wilson", "other"])))]
pub struct CommandArgs {
    ///Width of maze
    pub width: usize,

    ///height of maze
    pub height: usize,

    ///path of output png
    #[arg(long, default_value = "./out.png")]
    pub path_out: String,

    ///print to console instead of saving in a file
    #[arg(long, action)]
    pub console_print: bool,

    ///set the desired cell width in pixels
    #[arg(long, default_value = "10")]
    pub cell_width: u32,

    ///set the desired cell height in pixels
    #[arg(long, default_value = "10")]
    pub cell_height: u32,

    ///Should print solution on maze
    #[arg(long, action)]
    pub solve: bool,

    ///Serialize maze struct into <SERIALIZE>.json and <SERIALIZE>_path.json
    #[arg(long)]
    pub serialize: Option<String>,

    #[arg(long)]
    wilson: bool,

    #[arg(long)]
    other: bool,
}

impl CommandArgs {
    pub fn get_generator_type(&self) -> GeneratorType {
        if self.wilson {
            GeneratorType::Wilson
        } else if self.other {
            GeneratorType::Other
        } else {
            rand::random()
        }
    }
}

pub enum GeneratorType {
    Wilson,
    Other,
}

impl Distribution<GeneratorType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GeneratorType {
        match rng.gen_range(0..2) {
            0 => GeneratorType::Wilson,
            1 => GeneratorType::Other,
            _ => panic!("Error during random generation"),
        }
    }
}

pub fn main_run() {
    let config = CommandArgs::parse();
    let g_type = config.get_generator_type();
    let generator = get_maze_generator(g_type);
    let maze = MazeBuilder::from_generator(generator)
        .generate(config.width, config.height)
        .unwrap();

    if let Some(path_str) = config.serialize {
        let serialized = serde_json::to_string(&maze).unwrap();
        fs::write(format!("{path_str}.json",), serialized).expect("Failed to write file");

        let path = solver::solve_maze(&maze).unwrap();
        let serialized = serde_json::to_string(&path).unwrap();
        fs::write(format!("{path_str}_path.json"), serialized).expect("Failed to write to file");
    }

    if config.console_print {
        maze.print_to_console();
        exit(0);
    }

    let config_array = ConfigArray {
        cell_width: config.cell_width,
        cell_height: config.cell_height,
    };

    let mut image_builder = maze_image_builder::MazeImageBuilder::new(config_array, &maze);
    image_builder.solve(config.solve);
    let image = image_builder.build_image();
    image.save(config.path_out).expect("Can't save file");
}

fn get_maze_generator(generator_type: GeneratorType) -> Box<dyn MazeGenerator> {
    match generator_type {
        GeneratorType::Wilson => Box::new(WilsonGenerator),
        GeneratorType::Other => todo!("Implement Other generator"),
    }
}
