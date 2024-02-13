use ggez::{
    event::EventHandler,
    graphics::{self, Canvas, Color, DrawParam, Image},
    mint::Point2,
    Context, GameResult,
};

pub const SCREEN_SIZE: (f32, f32) = (1000.0, 700.0);
const TARGET_FPS: f64 = 60.0;

pub struct Ball {
    ball: Image,
}

impl Ball {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let ball = Image::from_path(ctx, "\\ball.png")?;

        Ok(Self { ball })
    }
}

pub struct GameState {
    ball: Ball,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(GameState {
            ball: Ball::new(ctx).unwrap(),
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(TARGET_FPS as u32) {
            
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::BLACK);

        canvas.draw(&self.ball.ball, DrawParam::default());

        canvas.finish(ctx)?;

        Ok(())
    }
}
