use ggez::{
    event::EventHandler,
    graphics::{self, Canvas, Color, DrawParam, Rect},
    Context, GameResult,
};

pub const SCREEN_SIZE: (f32, f32) = (1000.0, 700.0);

pub struct GameState {
    square: Rect,
}

impl GameState {
    pub fn new() -> GameResult<Self> {
        let square_length = 50.0;
        let square = Rect::new(
            SCREEN_SIZE.0 / 2.0 - square_length / 2.0,
            SCREEN_SIZE.1 / 4.0,
            square_length,
            square_length,
        );

        Ok(GameState { square })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(1) {}

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::BLACK);

        canvas.draw(
            &graphics::Quad,
            DrawParam::new()
            .dest_rect(self.square)
            .color(Color::WHITE),
        );

        canvas.finish(ctx)?;

        Ok(())
    }
}
