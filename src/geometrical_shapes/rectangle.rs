use raster::{Color, Image};

use super::line::draw_line;
use super::{Drawable, Point, random_color};

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub(crate) top_left: Point,
    pub(crate) bottom_right: Point,
    color: Color,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self {
            top_left: Point::new(p1.x, p1.y),
            bottom_right: Point::new(p2.x, p2.y),
            color: random_color(),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let top_right = Point::new(self.bottom_right.x, self.top_left.y);
        let bottom_left = Point::new(self.top_left.x, self.bottom_right.y);

        draw_line(image, &self.top_left, &top_right, &self.color());
        draw_line(image, &top_right, &self.bottom_right, &self.color());
        draw_line(image, &self.bottom_right, &bottom_left, &self.color());
        draw_line(image, &bottom_left, &self.top_left, &self.color());
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

