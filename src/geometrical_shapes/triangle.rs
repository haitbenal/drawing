use raster::{Color, Image};

use super::line::draw_line;
use super::{Drawable, Point, random_color};

#[derive(Debug, Clone)]
pub struct Triangle {
    pub(crate) a: Point,
    pub(crate) b: Point,
    pub(crate) c: Point,
    color: Color,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Self {
            a: a.clone(),
            b: b.clone(),
            c: c.clone(),
            color: random_color(),
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        draw_line(image, &self.a, &self.b, &self.color());
        draw_line(image, &self.b, &self.c, &self.color());
        draw_line(image, &self.c, &self.a, &self.color());
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
    fn triangle_draws_its_three_vertices() {
        let mut image = Image::blank(30, 30);
        let a = Point::new(10, 5);
        let b = Point::new(5, 25);
        let c = Point::new(25, 20);
        let triangle = Triangle::new(&a, &b, &c);
        triangle.draw(&mut image);

        for (x, y) in [(10, 5), (5, 25), (25, 20)] {
            let pixel = image.get_pixel(x, y).unwrap();
            assert_same_color(&triangle.color(), &pixel);
        }
    }

    #[test]
    fn triangle_edges_are_contiguous() {
        let mut image = Image::blank(30, 30);
        let triangle = Triangle::new(&Point::new(5, 5), &Point::new(25, 25), &Point::new(5, 25));
        triangle.draw(&mut image);

        // Vertical left edge from (5, 5) down to (5, 25).
        for y in 5..=25 {
            let pixel = image.get_pixel(5, y).unwrap();
            assert_same_color(&triangle.color(), &pixel);
        }
    }
}
