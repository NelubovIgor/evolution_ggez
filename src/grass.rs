use ggez::Context;
use ggez::input::keyboard::KeyCode;
use nalgebra::geometry::Point2;
use ggez::graphics::Color;

use crate::body::Body;

pub struct Grass {
    pub body: Body,
}

impl Grass {
    pub fn new(position: Point2<f32>, radius: Option<f32>, color: Option<Color>) -> Self {
        let radius = radius.unwrap_or(1.0);
        let color = color.unwrap_or(Color::GREEN);

        Self {
            body: Body::new(position, radius, color),
        }
    }
}
