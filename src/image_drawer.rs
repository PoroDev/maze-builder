use crate::maze::Coords;
use image::{GenericImage, Pixel};

///Implementing Bresenham for Images
pub trait GenericImageExt {
    type Pixel: Pixel;
    fn draw_line(&mut self, start: &Coords, end: &Coords, color: Self::Pixel);

    fn draw_line_with_thickness(
        &mut self,
        start: &Coords,
        end: &Coords,
        thickness: u32,
        color: Self::Pixel,
    );

    fn draw_full_square_with_center(&mut self, center: &Coords, radius: u32, color: Self::Pixel);
}

impl<T> GenericImageExt for T
where
    T: GenericImage,
{
    type Pixel = T::Pixel;

    fn draw_line(&mut self, start: &Coords, end: &Coords, color: Self::Pixel) {
        if !self.in_bounds(start.x as u32, start.y as u32)
            || !self.in_bounds(end.x as u32, end.y as u32)
        {
            return;
        };
        let dx = (end.x as i32 - start.x as i32).abs();
        let sx = if start.x < end.x { 1 } else { -1 };

        let dy = -(end.y as i32 - start.y as i32).abs();
        let sy = if start.y < end.y { 1 } else { -1 };

        let mut err = dx + dy;

        let mut x0 = start.x as i32;
        let mut y0 = start.y as i32;

        loop {
            self.put_pixel(x0 as u32, y0 as u32, color);

            if x0 == end.x as i32 && y0 == end.y as i32 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    fn draw_line_with_thickness(
        &mut self,
        start: &Coords,
        end: &Coords,
        thickness: u32,
        color: Self::Pixel,
    ) {
        if !self.in_bounds(start.x as u32, start.y as u32)
            || !self.in_bounds(end.x as u32, end.y as u32)
        {
            return;
        };
        let dx = (end.x as i32 - start.x as i32).abs();
        let sx = if start.x < end.x { 1 } else { -1 };

        let dy = -(end.y as i32 - start.y as i32).abs();
        let sy = if start.y < end.y { 1 } else { -1 };

        let mut err = dx + dy;

        let mut x0 = start.x as i32;
        let mut y0 = start.y as i32;

        loop {
            let coords = Coords {
                x: x0 as usize,
                y: y0 as usize,
            };
            self.draw_full_square_with_center(&coords, thickness, color);

            if x0 == end.x as i32 && y0 == end.y as i32 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    fn draw_full_square_with_center(&mut self, center: &Coords, radius: u32, color: Self::Pixel) {
        let r_pos = radius as i32 / 2;
        let r_neg = if radius % 2 == 0 { -r_pos + 1 } else { -r_pos };

        let x = center.x as i32;
        let y = center.y as i32;

        for dx in r_neg..=r_pos {
            for dy in r_neg..=r_pos {
                let x = u32::try_from(x + dx);
                let y = u32::try_from(y + dy);
                if x.is_err() || y.is_err() {
                    continue;
                }

                let (x, y) = (x.unwrap(), y.unwrap());
                if !self.in_bounds(x, y) {
                    continue;
                };
                self.put_pixel(x, y, color);
            }
        }
    }
}

#[cfg(test)]
mod tests_image_drawers {

    use super::*;
    use image::{Rgb, RgbImage};

    #[test]
    pub fn test_draw_rectangle() {
        let mut image = RgbImage::from_pixel(10, 10, Rgb { 0: [0x00; 3] });
        let begin = Coords { x: 2, y: 2 };
        let end = Coords { x: 2, y: 5 };
        image.draw_line_with_thickness(&begin, &end, 3, Rgb([0xff; 3]));
        image.save("temp_out.png").unwrap();
    }
}
