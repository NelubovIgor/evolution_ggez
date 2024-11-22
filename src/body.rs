use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, Mesh, DrawMode, DrawParam};
use nalgebra::geometry::Point2;

#[derive(Clone)]
pub struct Body {
    pub position: Point2<f32>,
    pub radius: f32,
    pub color: Color,
}

impl Body {
    pub fn new(position: Point2<f32>, radius: f32, color: Color) -> Self {
        Self {
            position,
            radius,
            color,
        }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        let mesh = Mesh::new_circle(
            ctx,
            DrawMode::fill(),
            [self.position.x, self.position.y],
            self.radius,
            1.0,
            self.color,
        )?;
        canvas.draw(&mesh, DrawParam::default());
        Ok(())
    }

    pub fn move_body(&mut self, dx: f32, dy: f32) {
        self.position.x += dx;
        self.position.y += dy;
    }
}
