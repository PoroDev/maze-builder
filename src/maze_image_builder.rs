use crate::image_drawer::GenericImageExt;
use crate::maze::{self, Coords, Maze};
use image::{Rgb, RgbImage};

pub struct ConfigArray {
    pub cell_width: u32,
    pub cell_height: u32,
}

pub struct MazeImageBuilder<'a> {
    config: ConfigArray,
    image: RgbImage,
    maze: &'a Maze,
    solve: bool,
}

impl<'a> MazeImageBuilder<'a> {
    pub fn new(config: ConfigArray, maze: &'a Maze) -> MazeImageBuilder<'a> {
        let (width, height) = MazeImageBuilder::get_image_array_size(maze, &config);
        let mut ret = MazeImageBuilder {
            config: config,
            maze: maze,
            image: RgbImage::from_pixel(width, height, Rgb { 0: [0xff; 3] }),
            solve: false,
        };
        ret.draw_maze();
        ret
    }

    pub fn get_image_array_size(maze: &Maze, config: &ConfigArray) -> (u32, u32) {
        let width_array = maze.get_width() as u32 * (config.cell_width - 1) + 1;
        let height_array = maze.get_height() as u32 * (config.cell_height - 1) + 1;

        (width_array as u32, height_array as u32)
    }

    pub fn draw_cell(&mut self, x: u32, y: u32) {
        let width_cell = self.config.cell_width;
        let height_cell = self.config.cell_height;

        let base_x = (width_cell - 1) * x;
        let base_y = (height_cell - 1) * y;
        let end_x = base_x + width_cell - 1;
        let end_y = base_y + height_cell - 1;

        let top_left_corner = Coords {
            x: base_x as usize,
            y: base_y as usize,
        };

        let top_right_corner = Coords {
            x: end_x as usize,
            y: base_y as usize,
        };

        let bottom_left_corner = Coords {
            x: base_x as usize,
            y: end_y as usize,
        };

        let bottom_right_corner = Coords {
            x: end_x as usize,
            y: end_y as usize,
        };

        let coords = Coords {
            x: x as usize,
            y: y as usize,
        };
        let cell = self.maze.borrow_cell(&coords);
        //draw top line
        if cell.top == maze::LinkType::Wall {
            self.image
                .draw_line(&top_left_corner, &top_right_corner, Rgb { 0: [0x00; 3] });
        }

        //draw left line
        if cell.left == maze::LinkType::Wall {
            self.image
                .draw_line(&top_left_corner, &bottom_left_corner, Rgb { 0: [0x00; 3] });
        }

        //draw right line
        if cell.right == maze::LinkType::Wall {
            self.image.draw_line(
                &top_right_corner,
                &bottom_right_corner,
                Rgb { 0: [0x00; 3] },
            );
        }

        //draw bottom line
        if cell.bottom == maze::LinkType::Wall {
            self.image.draw_line(
                &bottom_left_corner,
                &bottom_right_corner,
                Rgb { 0: [0x00; 3] },
            );
        }
    }

    pub fn build_image(mut self) -> RgbImage {
        self.draw_maze();
        self.image
    }

    fn draw_maze(&mut self) {
        let (width, height) = self.maze.get_dimensions();
        for y in 0..height {
            for x in 0..width {
                self.draw_cell(x as u32, y as u32);
            }
        }
    }

    pub fn solve(&mut self, flag: bool) {
        self.solve = flag;
    }
}
