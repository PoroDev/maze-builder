use crate::image_drawer::GenericImageExt;
use crate::maze::{self, Coords, Maze};
use crate::solver;
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
        let thickness_horizontal = (((width_cell as f32) * 0.1).round() as u32).max(1);
        let thickness_vertical = (((height_cell as f32) * 0.1).round() as u32).max(1);
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
            self.image.draw_line_with_thickness(
                &top_left_corner,
                &top_right_corner,
                thickness_vertical,
                Rgb { 0: [0x00; 3] },
            );
        }

        //draw left line
        if cell.left == maze::LinkType::Wall {
            self.image.draw_line_with_thickness(
                &top_left_corner,
                &bottom_left_corner,
                thickness_horizontal,
                Rgb { 0: [0x00; 3] },
            );
        }

        //draw right line
        if cell.right == maze::LinkType::Wall {
            self.image.draw_line_with_thickness(
                &top_right_corner,
                &bottom_right_corner,
                thickness_horizontal,
                Rgb { 0: [0x00; 3] },
            );
        }

        //draw bottom line
        if cell.bottom == maze::LinkType::Wall {
            self.image.draw_line_with_thickness(
                &bottom_left_corner,
                &bottom_right_corner,
                thickness_vertical,
                Rgb { 0: [0x00; 3] },
            );
        }
    }

    pub fn build_image(mut self) -> RgbImage {
        self.draw_maze();
        if self.solve {
            self.draw_solution();
        }
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

    fn draw_solution(&mut self) {
        let path = solver::solve_maze(&self.maze).unwrap();
        let width_cell = self.config.cell_width;
        let height_cell = self.config.cell_height;
        let thickness_horizontal = (((width_cell as f32) * 0.2).round() as u32).max(1);
        let thickness_vertical = (((height_cell as f32) * 0.2).round() as u32).max(1);
        let thickness = thickness_horizontal.max(thickness_vertical);
        let color = Rgb([0xff, 0x00, 0x00]);

        for i in 0..path.data.len() - 1 {
            let coords_first = path.data.get(i).unwrap();
            let coords_second = path.data.get(i + 1).unwrap();
            let base_x0 = (width_cell - 1) * coords_first.x as u32 + (width_cell - 1) / 2;
            let base_y0 = (height_cell - 1) * coords_first.y as u32 + (height_cell - 1) / 2;

            let base_x1 = (width_cell - 1) * coords_second.x as u32 + (width_cell - 1) / 2;
            let base_y1 = (height_cell - 1) * coords_second.y as u32 + (height_cell - 1) / 2;

            let begin = Coords {
                x: base_x0 as usize,
                y: base_y0 as usize,
            };

            let end = Coords {
                x: base_x1 as usize,
                y: base_y1 as usize,
            };

            self.image
                .draw_line_with_thickness(&begin, &end, thickness, color);
        }
    }

    pub fn solve(&mut self, flag: bool) {
        self.solve = flag;
    }
}
