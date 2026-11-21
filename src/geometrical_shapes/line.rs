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

