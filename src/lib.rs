use ggez::{
    event::EventHandler,
    graphics::{self, Canvas, Color, DrawParam, Rect},
    mint::Point2,
    Context, GameResult,
};

pub const SCREEN_SIZE: (f32, f32) = (1000.0, 700.0);

struct Square {
    square: Rect,
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
        }
    }

    fn move_position(&mut self, x: f32, y: f32) {
        self.square.x += x;
        self.square.y += y;
    }
}

pub struct GameState {
    square: Square,
}

impl GameState {
    pub fn new() -> GameResult<Self> {
        Ok(GameState {
            square: Square::new(),
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(60) {
            self.square.move_position(0.0, 5.0);
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

        let start_point = Point2 { x: 0.0, y: 600.0 };
        let end_point = Point2 { x: 1000.0, y: 600.0 };
        let line_thickness = 1.0;
        let line_color = Color::new(1.0, 1.0, 1.0, 1.0);
        let line_mesh = graphics::Mesh::new_line(ctx, &[start_point, end_point], line_thickness, line_color)?;

        canvas.draw(&line_mesh, graphics::DrawParam::default());

        canvas.finish(ctx)?;

        Ok(())
    }
}
