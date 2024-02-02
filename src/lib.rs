use ggez::{
    event::EventHandler,
    graphics::{self, Canvas, Color, DrawMode, DrawParam, Mesh, Rect},
    mint::Point2,
    Context, GameResult,
};
use std::f32::consts::PI;

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
        let y = SCREEN_SIZE.1 / 8.0;
        let corners = [
            Point2 { x, y },
            Point2 { x: x + length, y },
            Point2 { x, y: y + length },
            Point2 {
                x: x + length,
                y: y + length,
            },
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

struct Shape {
    shape: Mesh,
    corners: Vec<Point2<f32>>,
}

impl Shape {
    fn new(ctx: &mut Context) -> Shape {
        let length = 50.0;
        let x = 10.0;
        let y = SCREEN_SIZE.1 / 2.0;
        let corners = vec![
            Point2 { x, y },
            Point2 { x: x + length, y },
            Point2 { x, y: y + length },
            Point2 {
                x: x + length,
                y: y + length,
            },
        ];
        let shape = Mesh::new_polygon(ctx, DrawMode::fill(), &corners, Color::WHITE).unwrap();

        Shape { shape, corners }
    }

    fn new1(ctx: &mut Context) -> Shape {
        let length = 50.0;
        let x = SCREEN_SIZE.0 / 4.0;
        let y = SCREEN_SIZE.1 / 2.0;
        let corners = vec![
            Point2 { x, y },
            Point2 { x: x + length, y },
            Point2 {
                x: x + length,
                y: y + length,
            },
            Point2 { x, y: y + length },
        ];
        let shape = Mesh::new_polygon(ctx, DrawMode::fill(), &corners, Color::WHITE).unwrap();

        Shape { shape, corners }
    }

    fn new2(ctx: &mut Context) -> Shape {
        let length = 50.0;
        let x = SCREEN_SIZE.0 * 3.0 / 4.0;
        let y = SCREEN_SIZE.1 / 2.0;
        let corners = vec![
            Point2 { x, y },
            Point2 { x: x + length, y },
            Point2 {
                x: x + length / 2.0,
                y: y + length / 2.0,
            },
        ];
        let shape = Mesh::new_polygon(ctx, DrawMode::fill(), &corners, Color::WHITE).unwrap();

        Shape { shape, corners }
    }

    fn new3(ctx: &mut Context) -> Shape {
        let length = 50.0;
        let x = SCREEN_SIZE.0 - 100.0;
        let y = SCREEN_SIZE.1 / 2.0;
        let mut corners = Vec::new();
        for i in 0..6 {
            let angle = PI / 3.0 * i as f32;
            let x1 = x + length * angle.cos();
            let y1 = y + length * angle.sin();
            corners.push(Point2 { x: x1, y: y1 });
        }
        // let corners = vec![
        //     Point2 { x, y },
        //     Point2 { x: x + length, y },
        //     Point2 {
        //         x: x + length * (1.0 + angle.cos()),
        //         y: y + length * angle.sin(),
        //     },
        //     Point2 {
        //         x: x + length,
        //         y: y + 2.0 * length * angle.sin(),
        //     },
        //     Point2 {
        //         x: x,
        //         y: y + 2.0 * length * angle.sin(),
        //     },
        //     Point2 {
        //         x: x - length * angle.cos(),
        //         y: y + length * angle.sin(),
        //     },
        // ];
        let shape = Mesh::new_polygon(ctx, DrawMode::fill(), &corners, Color::WHITE).unwrap();

        Shape { shape, corners }
    }

    fn new_cirlce(ctx: &mut Context, pos: f32, tolerance: f32) -> Shape {
        let center = Point2 {
            x: SCREEN_SIZE.0 * pos / 8.0,
            y: SCREEN_SIZE.1 / 4.0,
        };
        let shape =
            Mesh::new_circle(ctx, DrawMode::fill(), center, 50.0, tolerance, Color::WHITE).unwrap();

        Shape {
            shape,
            corners: vec![],
        }
    }
}

pub struct GameState {
    // square: Square,
    floor: Floor,
    shape: Shape,
    shape1: Shape,
    shape2: Shape,
    shape3: Shape,
    circle1: Shape,
    // circle2: Shape,
    // circle3: Shape,
    // circle4: Shape,
    // circle5: Shape,
    // circle6: Shape,
    // circle7: Shape,
}

// TODO: Reconfigure gravity and collision detection to work with any shape
// TODO: Snap to floor
// TODO: Rotation oh god...
impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(GameState {
            // square: Square::new(),
            floor: Floor::new(ctx),
            shape: Shape::new(ctx),
            shape1: Shape::new1(ctx),
            shape2: Shape::new2(ctx),
            shape3: Shape::new3(ctx),
            circle1: Shape::new_cirlce(ctx, 4.0, 0.01),
            // circle2: Shape::new_cirlce(ctx, 2.0, 1.0),
            // circle3: Shape::new_cirlce(ctx, 3.0, 0.75),
            // circle4: Shape::new_cirlce(ctx, 4.0, 0.5),
            // circle5: Shape::new_cirlce(ctx, 5.0, 0.25),
            // circle6: Shape::new_cirlce(ctx, 6.0, 0.1),
            // circle7: Shape::new_cirlce(ctx, 7.0, 0.01),
        })
    }

    // fn square_intersects_floor(&self) -> bool {
    //     let start = self.floor.points[0];
    //     let end = self.floor.points[1];

    //     let x1 = start.x;
    //     let y1 = start.y;
    //     let x2 = end.x;
    //     let y2 = end.y;

    //     let x = self.square.square.x;
    //     let y = self.square.square.y;
    //     let w = self.square.square.w;
    //     let h = self.square.square.h;

    //     // Do the coordinates overlap at all
    //     if (x1 < x && x2 < x)
    //         || (x1 > x + w && x2 > x + w)
    //         || (y1 < y && y2 < y)
    //         || (y1 > y + h && y2 > y + h)
    //     {
    //         return false;
    //     }

    //     // Vertical Line
    //     if x1 == x2 {
    //         if x1 >= x && x2 <= x + w {
    //             return true;
    //         }
    //     }
    //     // Horizontal Line
    //     else if y1 == y2 {
    //         if y1 >= y && y2 <= y + h {
    //             return true;
    //         }
    //     } else {
    //         let m_floor = (y2 - y1) / (x2 - x1);
    //         let b_floor = y1 - m_floor * x1;

    //         for corner in self.square.corners {
    //             let m_square = -1.0 / m_floor;
    //             let b_square = corner.y - m_square * corner.x;

    //             let xi = (b_floor - b_square) / (m_square - m_floor);
    //             let yi = m_floor * xi + b_floor;

    //             if (x <= xi && xi <= x + w) && (y <= yi && yi <= y + h) {
    //                 return true;
    //             }
    //         }
    //     }

    //     false
    // }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(TARGET_FPS as u32) {
            // if self.square_intersects_floor() {
            //     self.square.velocity_x = 0.0;
            //     self.square.velocity_y = 0.0;
            // }
            // self.square
            //     .move_position(self.square.velocity_x as f32, self.square.velocity_y as f32);
            // self.square.apply_gravity(1.0 / TARGET_FPS);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::BLACK);

        // canvas.draw(
        //     &graphics::Quad,
        //     DrawParam::new()
        //         .dest_rect(self.square.square)
        //         .color(Color::WHITE),
        // );

        canvas.draw(&self.shape.shape, DrawParam::default());
        canvas.draw(&self.shape1.shape, DrawParam::default());
        canvas.draw(&self.shape2.shape, DrawParam::default());
        canvas.draw(&self.shape3.shape, DrawParam::default());
        canvas.draw(&self.circle1.shape, DrawParam::default());
        // canvas.draw(&self.circle2.shape, DrawParam::default());
        // canvas.draw(&self.circle3.shape, DrawParam::default());
        // canvas.draw(&self.circle4.shape, DrawParam::default());
        // canvas.draw(&self.circle5.shape, DrawParam::default());
        // canvas.draw(&self.circle6.shape, DrawParam::default());
        // canvas.draw(&self.circle7.shape, DrawParam::default());

        canvas.draw(&self.floor.line, DrawParam::default());

        canvas.finish(ctx)?;

        Ok(())
    }
}
