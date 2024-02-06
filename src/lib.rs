mod physics;

use ggez::{
    event::EventHandler, graphics::{self, Canvas, Color, DrawParam}, mint::Point2, Context, GameResult
};
pub use physics::{Circle, Floor, Gravity, Shape, Square};

pub const SCREEN_SIZE: (f32, f32) = (1000.0, 700.0);
const TARGET_FPS: f64 = 60.0;

fn dot(point1: &Point2<f32>, point2: &Point2<f32>) -> f32 {
    point1.x * point2.x + point1.y * point2.y
}

fn closest_point(start: &Point2<f32>, end: &Point2<f32>, t: f32, v: &Point2<f32>) -> Point2<f32> {
    if t <= 0.0 {
        *start
    } else if t >= 1.0 {
        *end
    } else {
        let x = start.x + t * v.x;
        let y = start.y + t * v.y;

        Point2 { x, y }
    }
}

fn distance(point1: &Point2<f32>, point2: &Point2<f32>) -> f32 {
    ((point2.x - point1.x).powi(2) + (point2.y - point1.y).powi(2)).sqrt()
}

pub struct GameState {
    square: Square,
    floor: Floor,
    shape: Shape,
    shape1: Shape,
    shape2: Shape,
    shape3: Shape,
    circle: Circle,
}

// TODO: Reconfigure collision detection to work with any shape
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
            circle: Circle::new(ctx),
        })
    }

    fn square_intersects_floor(&self, square: &Square) -> bool {
        let x = square.square.x;
        let y = square.square.y;
        let w = square.square.w;
        let h = square.square.h;

        for i in 0..self.floor.points.len() - 1 {
            let start = self.floor.points[i];
            let end = self.floor.points[i + 1];

            let x1 = start.x;
            let y1 = start.y;
            let x2 = end.x;
            let y2 = end.y;

            // Do the coordinates overlap at all
            if (x1 < x && x2 < x)
                || (x1 > x + w && x2 > x + w)
                || (y1 < y && y2 < y)
                || (y1 > y + h && y2 > y + h)
            {
                continue;
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
        }

        false
    }

    fn shape_intersects_floor(&self, shape: &Shape) -> bool {
        false
    }

    fn circle_intersects_floor(&self, circle: &Circle) -> bool {
        let center = circle.center;
        let radius = circle.radius;

        let left = center.x - radius;
        let right = center.x + radius;
        let top = center.y - radius;
        let bottom = center.y + radius;

        for i in 0..self.floor.points.len() - 1 {
            let start = self.floor.points[i];
            let end = self.floor.points[i + 1];

            if start.x < left && end.x < left
                || start.x > right && end.x > right
                || start.y > bottom && end.y > bottom
                || start.y < top && end.y < top
            {
                continue;
            }
            // Vertical line
            if start.x == end.x {
                if start.x >= left && end.x <= right {
                    return true;
                }
            }
            // Horizontal line
            if start.y == end.y {
                if start.y >= top && end.y <= bottom {
                    return true;
                }
            } else {
                let v = Point2 { x: end.x - start.x, y: end.y - start.y };
                let w = Point2 { x: center.x - start.x, y: center.y - start.y };

                let t = dot(&v, &w) / ((end.x - start.x).powi(2) + (end.y - start.y).powi(2));

                let closest_point = closest_point(&start, &end, t, &v);

                if distance(&center, &closest_point) <= radius {
                    return true;
                }
            }
        }

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
            if self.circle_intersects_floor(&self.circle) {
                self.circle.velocity_x = 0.0;
                self.circle.velocity_y = 0.0;
            }
            if self.shape_intersects_floor(&self.shape) {
                self.shape.velocity_x = 0.0;
                self.shape.velocity_y = 0.0;
            }
            if self.shape_intersects_floor(&self.shape1) {
                self.shape1.velocity_x = 0.0;
                self.shape1.velocity_y = 0.0;
            }
            if self.shape_intersects_floor(&self.shape2) {
                self.shape2.velocity_x = 0.0;
                self.shape2.velocity_y = 0.0;
            }
            if self.shape_intersects_floor(&self.shape3) {
                self.shape3.velocity_x = 0.0;
                self.shape3.velocity_y = 0.0;
            }

            self.square
                .move_position(self.square.velocity_x as f32, self.square.velocity_y as f32);
            self.shape.move_position(
                self.shape.velocity_x as f32,
                self.shape.velocity_y as f32,
                ctx,
            );
            self.shape1.move_position(
                self.shape1.velocity_x as f32,
                self.shape1.velocity_y as f32,
                ctx,
            );
            self.shape2.move_position(
                self.shape2.velocity_x as f32,
                self.shape2.velocity_y as f32,
                ctx,
            );
            self.shape3.move_position(
                self.shape3.velocity_x as f32,
                self.shape3.velocity_y as f32,
                ctx,
            );
            self.circle.move_position(
                self.circle.velocity_x as f32,
                self.circle.velocity_y as f32,
                ctx,
            );
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
