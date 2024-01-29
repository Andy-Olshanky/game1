use ggez::{
    event::EventHandler,
    graphics::{self, Canvas, Color, DrawParam, Mesh, Rect},
    mint::Point2,
    Context, GameResult,
};

pub const SCREEN_SIZE: (f32, f32) = (1000.0, 700.0);
const TARGET_FPS: f64 = 60.0;

trait Gravity {
    const ACCELERATION: f64 = 9.8;
    fn apply_gravity(&mut self, dt: f64);
}

struct Floor {
    line: Mesh,
    start_point: Point2<f32>,
    end_point: Point2<f32>,
    color: Color,
}

impl Floor {
    fn new(ctx: &mut Context) -> Floor {
        let start_point = Point2 { x: 0.0, y: 600.0 };
        let end_point = Point2 {
            x: SCREEN_SIZE.0,
            y: 600.0,
        };
        let color = Color::WHITE;
        let line = Mesh::new_line(ctx, &vec![start_point, end_point], 1.0, color).unwrap();
        
        Floor {
            line,
            start_point,
            end_point,
            color,
        }
    }
}

struct Square {
    square: Rect,
    velocity_x: f64,
    velocity_y: f64,
    _max_velocity_x: f64,
    max_velocity_y: f64,
}

impl Square {
    fn new() -> Square {
        let length = 50.0;

        Square {
            square: Rect::new(
                SCREEN_SIZE.0 / 2.0 - length / 2.0,
                SCREEN_SIZE.1 / 8.0,
                length,
                length,
            ),
            velocity_x: 0.0,
            velocity_y: 0.0,
            _max_velocity_x: 10.0,
            max_velocity_y: 10.0,
        }
    }

    fn move_position(&mut self, x: f32, y: f32) {
        self.square.x += x;
        self.square.y += y;
    }
}

impl Gravity for Square {
    fn apply_gravity(&mut self, dt: f64) {
        self.velocity_y += Self::ACCELERATION * dt;
        if self.velocity_y > self.max_velocity_y {
            self.velocity_y = self.max_velocity_y;
        }
    }
}

pub struct GameState {
    square: Square,
    floor: Floor,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(GameState {
            square: Square::new(),
            floor: Floor::new(ctx),
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(TARGET_FPS as u32) {
            self.square
                .move_position(self.square.velocity_x as f32, self.square.velocity_y as f32);
            self.square.apply_gravity(1.0 / TARGET_FPS);

        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::BLACK);

        canvas.draw(
            &graphics::Quad,
            DrawParam::new()
                .dest_rect(self.square.square)
                .color(Color::WHITE),
        );

        canvas.draw(&self.floor.line, graphics::DrawParam::default());

        canvas.finish(ctx)?;

        Ok(())
    }
}
