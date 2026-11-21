use raster::{Color, Image};

use super::{Drawable, Point, display_pixel, random_color};

#[derive(Debug, Clone)]
pub struct Line {
    pub(crate) from: Point,
    pub(crate) to: Point,
    color: Color,
}

impl Line {
    pub fn new(from: &Point, to: &Point) -> Self {
        Self {
            from: from.clone(),
            to: to.clone(),
            color: random_color(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Self::new(&Point::random(width, height), &Point::random(width, height))
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        draw_line(image, &self.from, &self.to, &self.color());
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

/// Draws a one pixel wide line between two points using Bresenham's algorithm.
pub(crate) fn draw_line(image: &mut Image, from: &Point, to: &Point, color: &Color) {
    let (mut x0, mut y0) = (from.x, from.y);
    let (x1, y1) = (to.x, to.y);

    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        display_pixel(image, x0, y0, color.clone());
        if x0 == x1 && y0 == y1 {
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

#[cfg(test)]
mod tests {
    use super::super::assert_same_color;
    use super::*;

    #[test]
    fn random_lines_stay_within_dimensions() {
        for _ in 0..1000 {
            let line = Line::random(640, 480);
            assert!(line.from.x >= 0 && line.from.x < 640);
            assert!(line.from.y >= 0 && line.from.y < 480);
            assert!(line.to.x >= 0 && line.to.x < 640);
            assert!(line.to.y >= 0 && line.to.y < 480);
        }
    }

    #[test]
    fn line_with_identical_points_draws_a_single_pixel() {
        let mut image = Image::blank(10, 10);
        let line = Line::new(&Point::new(5, 5), &Point::new(5, 5));
        line.draw(&mut image);

        let mut drawn = 0;
        for y in 0..image.height {
            for x in 0..image.width {
                let pixel = image.get_pixel(x, y).unwrap();
                if pixel.r != 0 || pixel.g != 0 || pixel.b != 0 {
                    drawn += 1;
                }
            }
        }

        assert_eq!(drawn, 1);
        let pixel = image.get_pixel(5, 5).unwrap();
        assert_same_color(&line.color(), &pixel);
    }

    #[test]
    fn horizontal_line_is_contiguous() {
        let mut image = Image::blank(20, 20);
        let line = Line::new(&Point::new(2, 7), &Point::new(17, 7));
        line.draw(&mut image);

        for x in 2..=17 {
            let pixel = image.get_pixel(x, 7).unwrap();
            assert_same_color(&line.color(), &pixel);
        }
    }
}
