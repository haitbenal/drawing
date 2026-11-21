use raster::{Color, Image};

use super::{Drawable, display_pixel, random_color, random_coordinate};

#[derive(Debug, Clone)]
pub struct Point {
    pub(crate) x: i32,
    pub(crate) y: i32,
    color: Color,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            color: random_color(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Self::new(random_coordinate(width), random_coordinate(height))
    }
}
impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        display_pixel(image, self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_points_stay_within_dimensions() {
        for _ in 0..1000 {
            let point = Point::random(1000, 800);
            assert!(point.x >= 0 && point.x < 1000);
            assert!(point.y >= 0 && point.y < 800);
        }
    }

    #[test]
    fn random_points_stay_within_tiny_dimensions() {
        for _ in 0..100 {
            let point = Point::random(1, 1);
            assert!(point.x >= 0 && point.x < 1);
            assert!(point.y >= 0 && point.y < 1);
        }
    }
}
