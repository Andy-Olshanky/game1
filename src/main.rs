use game1::{GameState, SCREEN_SIZE};
use ggez::{event, ContextBuilder, GameResult};

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("first game", "Andy Olshansky")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .window_setup(ggez::conf::WindowSetup::default().title("First Game :)"))
        .build()?;

    let state = GameState::new()?;

    event::run(ctx, event_loop, state);
}
