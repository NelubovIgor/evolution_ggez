use ggez::Context;
use ggez::input::keyboard::KeyCode;
use nalgebra::geometry::Point2;
use ggez::graphics::Color;

use crate::body::Body;

pub struct Agent {
    pub body: Body,
    pub speed: f32,
}

impl Agent {
    pub fn new(position: Point2<f32>, radius: Option<f32>, color: Option<Color>, speed: Option<f32>) -> Self {
        let radius = radius.unwrap_or(1.0);
        let color = color.unwrap_or(Color::BLUE);
        let speed = speed.unwrap_or(1.0);

        Self {
            body: Body::new(position, radius, color),
            speed,
        }
    }

    pub fn move_agent(&mut self, dx: f32, dy: f32, bodies: &mut Vec<Body>) {
        self.body.move_body(dx, dy);
        
        if let Some(body) = bodies.iter_mut().find(|b| b.position == self.body.position) {
            body.move_body(dx, dy);
            // println!("moved!!!");
        }
    
    }

    pub fn move_player(&mut self, ctx: &Context, bodies: &mut Vec<Body>) {
        let mut dx = 0.0;
        let mut dy = 0.0;

        if ctx.keyboard.is_key_pressed(KeyCode::D) {
            dx += self.speed;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::A) {
            dx -= self.speed;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::S) {
            dy += self.speed;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::W) {
            dy -= self.speed;
        }

        if dx != 0.0 || dy != 0.0 {
            self.body.move_body(dx, dy);
            // self.move_agent(dx, dy, bodies);
        }
    }
}
