use ggez::{event::EventHandler, graphics::{self, Canvas}, Context, GameResult};

pub struct GameState {
    inc: i32,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(GameState {
            inc: 0
        })
    }

    pub fn draw_thing(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas: Canvas;
        if self.inc % 2 == 1 {
            canvas = Canvas::from_frame(ctx, graphics::Color::BLACK);
        }
        else {
            canvas = Canvas::from_frame(ctx, graphics::Color::WHITE);
        }

        canvas.finish(ctx)?;

        Ok(())
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(1) {
            self.inc += 1;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.draw_thing(ctx)?;

        Ok(())
    }
}