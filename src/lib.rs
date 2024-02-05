mod physics;

pub use physics::{Gravity, Square, Shape, Circle, Floor};
use ggez::{
    event::EventHandler,
    graphics::{self, Canvas, Color, DrawParam},
    Context, GameResult,
};

pub const SCREEN_SIZE: (f32, f32) = (1000.0, 700.0);
const TARGET_FPS: f64 = 60.0;

pub struct GameState {
    square: Square,
    floor: Floor,
    shape: Shape,
    shape1: Shape,
    shape2: Shape,
    shape3: Shape,
    circle: Circle
}

// TODO: Reconfigure gravity and collision detection to work with any shape
// TODO: Snap to floor
// TODO: Rotation oh god...
impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(GameState {
            square: Square::new(),
            floor: Floor::new(ctx),
            shape: Shape::new(ctx),
            shape1: Shape::new1(ctx),
            shape2: Shape::new2(ctx),
            shape3: Shape::new3(ctx),
            circle: Circle::new(ctx)
        })
    }

    fn square_intersects_floor(&self, square: &Square) -> bool {
        let start = self.floor.points[0];
        let end = self.floor.points[1];

        let x1 = start.x;
        let y1 = start.y;
        let x2 = end.x;
        let y2 = end.y;

        let x = square.square.x;
        let y = square.square.y;
        let w = square.square.w;
        let h = square.square.h;

        // Do the coordinates overlap at all
        if (x1 < x && x2 < x)
            || (x1 > x + w && x2 > x + w)
            || (y1 < y && y2 < y)
            || (y1 > y + h && y2 > y + h)
        {
            return false;
        }

        // Vertical Line
        if x1 == x2 {
            if x1 >= x && x2 <= x + w {
                return true;
            }
        }
        // Horizontal Line
        else if y1 == y2 {
            if y1 >= y && y2 <= y + h {
                return true;
            }
        } else {
            let m_floor = (y2 - y1) / (x2 - x1);
            let b_floor = y1 - m_floor * x1;

            for corner in square.corners {
                let m_square = -1.0 / m_floor;
                let b_square = corner.y - m_square * corner.x;

                let xi = (b_floor - b_square) / (m_square - m_floor);
                let yi = m_floor * xi + b_floor;

                if (x <= xi && xi <= x + w) && (y <= yi && yi <= y + h) {
                    return true;
                }
            }
        }

        false
    }

    fn shape_intersects_floor(&self, shape: &Shape) -> bool {
        false
    }
    
    fn circle_intersects_floor(&self, shape: &Circle) -> bool {
        false
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(TARGET_FPS as u32) {
            self.shape.apply_gravity(1.0 / TARGET_FPS);
            self.shape1.apply_gravity(1.0 / TARGET_FPS);
            self.shape2.apply_gravity(1.0 / TARGET_FPS);
            self.shape3.apply_gravity(1.0 / TARGET_FPS);
            self.circle.apply_gravity(1.0 / TARGET_FPS);
            self.square.apply_gravity(1.0 / TARGET_FPS);

            if self.square_intersects_floor(&self.square) {
                self.square.velocity_x = 0.0;
                self.square.velocity_y = 0.0;
            }
            self.square
                .move_position(self.square.velocity_x as f32, self.square.velocity_y as f32);
            
            self.shape.move_position(self.shape.velocity_x as f32, self.shape.velocity_y as f32, ctx);
            self.shape1.move_position(self.shape.velocity_x as f32, self.shape.velocity_y as f32, ctx);
            self.shape2.move_position(self.shape.velocity_x as f32, self.shape.velocity_y as f32, ctx);
            self.shape3.move_position(self.shape.velocity_x as f32, self.shape.velocity_y as f32, ctx);
            self.circle.move_position(self.shape.velocity_x as f32, self.shape.velocity_y as f32, ctx);
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

        canvas.draw(&self.shape.shape, DrawParam::default());
        canvas.draw(&self.shape1.shape, DrawParam::default());
        canvas.draw(&self.shape2.shape, DrawParam::default());
        canvas.draw(&self.shape3.shape, DrawParam::default());
        canvas.draw(&self.circle.circle, DrawParam::default());
        canvas.draw(&self.floor.line, DrawParam::default());

        canvas.finish(ctx)?;

        Ok(())
    }
}
