use rand::Rng;
use raster::{Color, Image};

use super::{Drawable, Point, display_pixel, random_color};

#[derive(Debug, Clone)]
pub struct Circle {
    pub(crate) center: Point,
    pub(crate) radius: i32,
    color: Color,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Self {
            center: center.clone(),
            radius,
            color: random_color(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let center = Point::random(width, height);
        let max_radius = width.min(height) / 8;
        let radius = if max_radius < 2 {
            1
        } else {
            rand::thread_rng().gen_range(2..=max_radius)
        };
        Self::new(&center, radius)
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        draw_circle(
            image,
            self.center.x,
            self.center.y,
            self.radius,
            &self.color(),
        );
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}

/// Draws the outline of a circle using the midpoint circle algorithm.
pub(crate) fn draw_circle(
    image: &mut Image,
    center_x: i32,
    center_y: i32,
    radius: i32,
    color: &Color,
) {
    let radius = radius.abs();
    if radius == 0 {
        display_pixel(image, center_x, center_y, color.clone());
        return;
    }

    let mut x = radius;
    let mut y = 0;
    let mut err = 1 - radius;

    while x >= y {
        let points = [
            (center_x + x, center_y + y),
            (center_x + y, center_y + x),
            (center_x - y, center_y + x),
            (center_x - x, center_y + y),
            (center_x - x, center_y - y),
            (center_x - y, center_y - x),
            (center_x + y, center_y - x),
            (center_x + x, center_y - y),
        ];
        for (px, py) in points {
            display_pixel(image, px, py, color.clone());
        }

        y += 1;
        if err < 0 {
            err += 2 * y + 1;
        } else {
            x -= 1;
            err += 2 * (y - x) + 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::assert_same_color;
    use super::*;

    #[test]
    fn random_circles_stay_within_dimensions() {
        for _ in 0..1000 {
            let circle = Circle::random(320, 200);
            assert!(circle.center.x >= 0 && circle.center.x < 320);
            assert!(circle.center.y >= 0 && circle.center.y < 200);
            assert!(circle.radius > 0);
        }
    }

    #[test]
    fn random_circles_get_a_positive_radius_on_tiny_images() {
        for _ in 0..100 {
            let circle = Circle::random(1, 1);
            assert_eq!(circle.radius, 1);
        }
    }

    #[test]
    fn circle_draws_pixels_on_its_outline_only() {
        let mut image = Image::blank(50, 50);
        let circle = Circle::new(&Point::new(25, 25), 10);
        circle.draw(&mut image);

        // The center of an outlined circle stays untouched.
        let center = image.get_pixel(25, 25).unwrap();
        assert_eq!((center.r, center.g, center.b), (0, 0, 0));

        // The four extreme points of the outline are drawn.
        for (x, y) in [(35, 25), (15, 25), (25, 35), (25, 15)] {
            let pixel = image.get_pixel(x, y).unwrap();
            assert_same_color(&circle.color(), &pixel);
        }
    }
}
