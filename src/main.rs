mod body;
mod agent;
mod constants;
mod grass;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Mesh, Drawable, DrawMode, DrawParam, Rect, Text};
use ggez::event::{self, EventHandler};
use ggez::input::keyboard::{KeyCode, KeyInput, KeyboardContext};
use nalgebra::geometry::Point2;
// use std::collections::HashSet;
// use ordered_float::OrderedFloat;
use rand::Rng;

use crate::body::Body;
use crate::agent::Agent;
use crate::grass::Grass;

// type OrderedPosition = (OrderedFloat<f32>, OrderedFloat<f32>);

// fn generate_random_position(width: f32, height: f32, occupied_positions: &HashSet<OrderedPosition>) -> Point2<f32> {
//     let mut rng = rand::thread_rng();

//     loop {
//         let x = rng.gen_range(0.0..width);
//         let y = rng.gen_range(0.0..height);
//         let ordered_pos = (OrderedFloat(x), OrderedFloat(y));
//         if !occupied_positions.contains(&ordered_pos) {
//             return Point2::new(x, y);
//         }
//     }
// }

fn random_point() -> Point2<f32> {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0.0..constants::MAIN_WIDTH);
    let y = rng.gen_range(0.0..constants::MAIN_HEIGHT);

    return Point2::new(x, y);
}

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(ggez::conf::WindowMode::default().dimensions(constants::MAIN_WIDTH, constants::MAIN_HEIGHT))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    cycles: u32,
    agents: Vec<Agent>,
    grasses: Vec<Grass>,
    player: Agent,
    paused: bool,
    bodies: Vec<Body>,
    // occupied_positions: HashSet<OrderedPosition>,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        let paused = true;
        // let mut occupied_positions: HashSet<OrderedPosition>  = HashSet::new();
        let mut agents = Vec::with_capacity(200);
        let mut grasses = Vec::with_capacity(200);
        let mut bodies = Vec::new();


        let point_player = Point2::new(100.0, 100.0);
        let player: Agent = Agent::new(point_player, Some(2.0), Some(Color::RED), Some(1.5));
        // occupied_positions.insert((OrderedFloat(point_player.x), OrderedFloat(point_player.y)));

        for i in 0..20 {
            let position: Point2<f32> = random_point();
            grasses.push(Grass::new(position, None, None));
        }

        for i in 0..20 {
            let position: Point2<f32> = random_point();
            // let position: Point2<f32> = generate_random_position(constants::MAIN_WIDTH, constants::MAIN_HEIGHT, &occupied_positions);      
            // occupied_positions.insert((OrderedFloat(position.x), OrderedFloat(position.y)));
            agents.push(Agent::new(position, None, None, None));
        }

        // bodies.push(player.body.clone());
        for agent in &agents {
            bodies.push(agent.body.clone());
        }

        for grass in &grasses {
            bodies.push(grass.body.clone());
        }

        MyGame {
            paused,
            cycles: 0,
            player,
            agents,
            bodies,
            grasses,
            // occupied_positions,
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.paused {
            return  Ok(());
        }

        self.player.move_player(ctx, &mut self.bodies);
        
        if self.cycles % 30 == 0 {
            let position: Point2<f32> = random_point();
            self.grasses.push(Grass::new(position, None, None));
        }

        self.cycles += 1;

        // println!("{}", self.player.body.position.x);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // for body in &self.bodies {
        //     body.draw(ctx, &mut canvas)?;
        // }

        for agent in &self.agents {
            agent.body.draw(ctx, &mut canvas)?;
        }

        for grass in &self.grasses {
            grass.body.draw(ctx, &mut canvas)?;
        }

        self.player.body.draw(ctx, &mut canvas);

        let sidebar_width = 200.0;
        let sidebar_color = Color::from_rgb(50, 50, 50);
        let sidebar_rect = Rect::new(
            constants::MAIN_WIDTH - sidebar_width,
            0.0,
            sidebar_width,
            constants::MAIN_HEIGHT,
        );
        let sidebar_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            sidebar_rect,
            sidebar_color,
        )?;    
        canvas.draw(&sidebar_mesh, DrawParam::default());
 
        let cycles_text = Text::new(format!("Cycles: {}", self.cycles));
        let text_color = Color::WHITE;
        let text_position = Rect::new(constants::MAIN_WIDTH - sidebar_width, 0.0, 1.0, 1.0);
        canvas.draw(&cycles_text, DrawParam::default().dest_rect(text_position).color(text_color));

        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::Space) => {self.paused = !self.paused;}
            Some(KeyCode::D) => {}
            Some(KeyCode::A) => {}
            Some(KeyCode::S) => {}
            Some(KeyCode::W) => {}
            _ => (),
        }
        Ok(())
    }
}