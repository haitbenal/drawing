use rand::Rng;
use raster::{Color, Image};

mod circle;
mod line;
mod point;
mod rectangle;
mod triangle;

pub use circle::Circle;
pub use line::Line;
pub use point::Point;
pub use rectangle::Rectangle;
pub use triangle::Triangle;

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

#[allow(dead_code)]
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub(crate) fn display_pixel(image: &mut Image, x: i32, y: i32, color: Color) {
    if x >= 0 && x < image.width && y >= 0 && y < image.height {
        image.set_pixel(x, y, color).unwrap();
    }
}

#[cfg(test)]
pub(crate) fn assert_same_color(expected: &Color, actual: &Color) {
    assert_eq!(
        (expected.r, expected.g, expected.b, expected.a),
        (actual.r, actual.g, actual.b, actual.a)
    );
}

pub(crate) fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::rgb(
        rng.gen_range(100..=255),
        rng.gen_range(100..=255),
        rng.gen_range(100..=255),
    )
}

pub(crate) fn random_coordinate(max: i32) -> i32 {
    if max <= 0 {
        0
    } else {
        rand::thread_rng().gen_range(0..max)
    }
}
