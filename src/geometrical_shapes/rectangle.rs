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
            top_left: Point::new(p1.x.min(p2.x), p1.y.min(p2.y)),
            bottom_right: Point::new(p1.x.max(p2.x), p1.y.max(p2.y)),
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

#[cfg(test)]
mod tests {
    use super::super::assert_same_color;
    use super::*;

    #[test]
    fn rectangle_normalizes_reversed_points() {
        let rectangle = Rectangle::new(&Point::new(150, 300), &Point::new(50, 60));

        assert_eq!(rectangle.top_left.x, 50);
        assert_eq!(rectangle.top_left.y, 60);
        assert_eq!(rectangle.bottom_right.x, 150);
        assert_eq!(rectangle.bottom_right.y, 300);
    }

    #[test]
    fn rectangle_keeps_ordered_points() {
        let rectangle = Rectangle::new(&Point::new(10, 20), &Point::new(30, 40));

        assert_eq!(rectangle.top_left.x, 10);
        assert_eq!(rectangle.top_left.y, 20);
        assert_eq!(rectangle.bottom_right.x, 30);
        assert_eq!(rectangle.bottom_right.y, 40);
    }

    #[test]
    fn rectangle_draws_outline_and_leaves_center_empty() {
        let mut image = Image::blank(40, 40);
        let rectangle = Rectangle::new(&Point::new(10, 10), &Point::new(30, 30));
        rectangle.draw(&mut image);

        for (x, y) in [(10, 10), (30, 10), (30, 30), (10, 30)] {
            let pixel = image.get_pixel(x, y).unwrap();
            assert_same_color(&rectangle.color(), &pixel);
        }

        let center = image.get_pixel(20, 20).unwrap();
        assert_eq!((center.r, center.g, center.b), (0, 0, 0));
    }
}
