use crate::maze::{self, Maze};
use image::{GrayImage, Luma};

pub struct ConfigArray {
    pub cell_width: u32,
    pub cell_height: u32,
}

pub struct MazeImageBuilder<'a> {
    config: ConfigArray,
    image: GrayImage,
    maze: &'a Maze,
}

impl<'a> MazeImageBuilder<'a> {
    pub fn new(config: ConfigArray, maze: &'a Maze) -> MazeImageBuilder<'a> {
        let (width, height) = MazeImageBuilder::get_image_array_size(maze, &config);
        MazeImageBuilder {
            config: config,
            maze: maze,
            image: GrayImage::from_pixel(width, height, Luma { 0: [0xff] }),
        }
    }

    pub fn get_image_array_size(maze: &Maze, config: &ConfigArray) -> (u32, u32) {
        let width_array = maze.width as u32 * (config.cell_width - 1) + 1;
        let height_array = maze.height as u32 * (config.cell_height - 1) + 1;

        (width_array as u32, height_array as u32)
    }

    pub fn draw_cell(&mut self, x: u32, y: u32) {
        let width_cell = self.config.cell_width;
        let height_cell = self.config.cell_height;

        let base_x = (width_cell - 1) * x;
        let base_y = (height_cell - 1) * y;

        let cell = self.maze.borrow_cell(x as usize, y as usize);
        //draw top line
        if cell.top == maze::LinkType::Wall {
            for i in 0..width_cell {
                self.image.put_pixel(base_x + i, base_y, Luma { 0: [0x00] });
            }
        }

        //draw left line
        if cell.left == maze::LinkType::Wall {
            for i in 0..height_cell {
                self.image.put_pixel(base_x, base_y + i, Luma { 0: [0x00] });
            }
        }

        //draw right line
        if cell.right == maze::LinkType::Wall {
            for i in 0..height_cell {
                self.image
                    .put_pixel(base_x + width_cell - 1, base_y + i, Luma { 0: [0x00] });
            }
        }

        //draw bottom line
        if cell.bottom == maze::LinkType::Wall {
            for i in 0..width_cell {
                self.image
                    .put_pixel(base_x + i, base_y + height_cell - 1, Luma { 0: [0x00] });
            }
        }
    }

    pub fn build_image(&mut self) -> &GrayImage {
        for y in 0..self.maze.height {
            for x in 0..self.maze.width {
                self.draw_cell(x as u32, y as u32);
            }
        }
        &self.image
    }
}
