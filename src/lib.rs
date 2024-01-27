use ggez::{event::EventHandler, graphics::{self, Canvas}, Context, GameResult};

pub struct GameState {}

impl GameState {
    pub fn new() -> GameResult<Self> {
        Ok(GameState {})
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(1) {
            
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let canvas = Canvas::from_frame(ctx, graphics::Color::BLACK);

        canvas.finish(ctx)?;

        Ok(())
    }
}