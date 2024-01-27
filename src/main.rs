use game1::GameState;
use ggez::{event, ContextBuilder, GameResult};

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("first game", "Andy Olshansky")
        .window_mode(ggez::conf::WindowMode::default().dimensions(1000.0, 700.0))
        .window_setup(ggez::conf::WindowSetup::default().title("First Game :)"))
        .build()?;

    let state = GameState::new()?;

    event::run(ctx, event_loop, state);
}
