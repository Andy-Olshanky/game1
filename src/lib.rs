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
    points: Vec<Point2<f32>>,
}

impl Floor {
    fn new(ctx: &mut Context) -> Floor {
        let points = vec![
            Point2 { x: 0.0, y: 600.0 },
            Point2 {
                x: SCREEN_SIZE.0,
                y: 600.0,
            },
        ];
        let line = Mesh::new_line(ctx, &points, 1.0, Color::WHITE).unwrap();

        Floor { line, points }
    }
}

struct Square {
    square: Rect,
    velocity_x: f64,
    velocity_y: f64,
    _max_velocity_x: f64,
    max_velocity_y: f64,
    corners: [Point2<f32>; 4],
}

impl Square {
    fn new() -> Square {
        let length = 50.0;
        let x = SCREEN_SIZE.0 / 2.0 - length / 2.0;
        let y = SCREEN_SIZE.1  / 2.0;
        let corners = [
            Point2 { x, y },
            Point2 { x: x + length, y },
            Point2 { x, y: y + length },
            Point2 { x: x + length, y: y + length },
        ];

        Square {
            square: Rect::new(x, y, length, length),
            velocity_x: 0.0,
            velocity_y: 0.0,
            _max_velocity_x: 15.0,
            max_velocity_y: 15.0,
            corners,
        }
    }

    fn move_position(&mut self, x: f32, y: f32) {
        self.square.x += x;
        self.square.y += y;
        self.update_corners();
    }

    fn update_corners(&mut self) {
        self.corners[0].x = self.square.x;
        self.corners[0].y = self.square.y;

        self.corners[1].x = self.square.x + self.square.w;
        self.corners[1].y = self.square.y;

        self.corners[2].x = self.square.x;
        self.corners[2].y = self.square.y + self.square.h;

        self.corners[3].x = self.square.x + self.square.w;
        self.corners[3].y = self.square.y + self.square.h
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

// TODO: Snap to floor
// TODO: Rotation oh god...
impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(GameState {
            square: Square::new(),
            floor: Floor::new(ctx),
        })
    }

    fn square_intersects_floor(&self) -> bool {
        let start = self.floor.points[0];
        let end = self.floor.points[1];

        let x1 = start.x;
        let y1 = start.y;
        let x2 = end.x;
        let y2 = end.y;

        let x = self.square.square.x;
        let y = self.square.square.y;
        let w = self.square.square.w;
        let h = self.square.square.h;

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

            for corner in self.square.corners {
                let m_square = -1.0 / m_floor;
                let b_square = corner.y - m_square * corner.x;

                let xi = (b_floor - b_square) / (m_square - m_floor);
                let yi = m_floor * xi + b_floor;

                if (x <= xi && xi <= x + w) && (y <= yi && yi <= y + h) {
                    return true;
                }
            }
            // let m_square = -1.0 / m_floor;
            // let b_square = y - m_square * x;

            // let xi = (b_floor - b_square) / (m_square - m_floor);
            // let yi = m_floor * xi + b_floor;

            // if (x <= xi && xi <= x + w) && (y <= yi && yi <= y + h) {
            //     return true;
            // }
        }

        false
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(TARGET_FPS as u32) {
            if self.square_intersects_floor() {
                self.square.velocity_x = 0.0;
                self.square.velocity_y = 0.0;
            }
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
