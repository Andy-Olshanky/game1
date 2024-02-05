use ggez::{
    graphics::{Color, DrawMode, Mesh, Rect},
    mint::Point2,
    Context,
};
use std::f32::consts::PI;
use crate::SCREEN_SIZE;

pub trait Gravity {
    const ACCELERATION: f64 = 9.8;
    fn apply_gravity(&mut self, dt: f64);
}

pub struct Floor {
    pub line: Mesh,
    pub points: Vec<Point2<f32>>,
}

impl Floor {
    pub fn new(ctx: &mut Context) -> Floor {
        // // Flat on bottom
        // let points = vec![
        //     Point2 { x: 0.0, y: 600.0 },
        //     Point2 {
        //         x: SCREEN_SIZE.0,
        //         y: 600.0,
        //     },
        // ];

        // // Flat on top
        // let points = vec![
        //     Point2 { x: 0.0, y: 100.0 },
        //     Point2 {
        //         x: SCREEN_SIZE.0,
        //         y: 100.0,
        //     },
        // ];

        // // Flat on right
        // let points = vec![
        //     Point2 { x: 900.0, y: 0.0 },
        //     Point2 {
        //         x: 900.0,
        //         y: SCREEN_SIZE.1,
        //     },
        // ];

        // // Flat on left
        // let points = vec![
        //     Point2 { x: 100.0, y: 0.0 },
        //     Point2 {
        //         x: 100.0,
        //         y: SCREEN_SIZE.1,
        //     },
        // ];

        // Positive on bottom
        let points = vec![
            Point2 { x: 0.0, y: 700.0 },
            Point2 {
                x: SCREEN_SIZE.0,
                y: 600.0,
            },
        ];

        // // Negative on bottom
        // let points = vec![
        //     Point2 { x: 0.0, y: 600.0 },
        //     Point2 {
        //         x: SCREEN_SIZE.0,
        //         y: 700.0,
        //     },
        // ];

        // // Positive on top
        // let points = vec![
        //     Point2 { x: 0.0, y: 100.0 },
        //     Point2 {
        //         x: SCREEN_SIZE.0,
        //         y: 0.0,
        //     },
        // ];

        // // Negative on top
        // let points = vec![
        //     Point2 { x: 0.0, y: 0.0 },
        //     Point2 {
        //         x: SCREEN_SIZE.0,
        //         y: 100.0,
        //     },
        // ];

        // // Positive on right
        // let points = vec![
        //     Point2 { x: 900.0, y: 0.0 },
        //     Point2 {
        //         x: 1000.0,
        //         y: SCREEN_SIZE.1,
        //     },
        // ];

        // // Negative on right
        // let points = vec![
        //     Point2 { x: 1000.0, y: 0.0 },
        //     Point2 {
        //         x: 900.0,
        //         y: SCREEN_SIZE.1,
        //     },
        // ];

        // // Positive on left
        // let points = vec![
        //     Point2 { x: 0.0, y: 0.0 },
        //     Point2 {
        //         x: 100.0,
        //         y: SCREEN_SIZE.1,
        //     },
        // ];

        // // Negative on left
        // let points = vec![
        //     Point2 { x: 100.0, y: 0.0 },
        //     Point2 {
        //         x: 0.0,
        //         y: SCREEN_SIZE.1,
        //     },
        // ];

        let line = Mesh::new_line(ctx, &points, 1.0, Color::WHITE).unwrap();

        Floor { line, points }
    }
}

pub struct Square {
    pub square: Rect,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub _max_velocity_x: f64,
    pub max_velocity_y: f64,
    pub corners: [Point2<f32>; 4],
}

impl Square {
    pub fn new() -> Square {
        let length = 50.0;
        let x = SCREEN_SIZE.0 / 2.0 - length / 2.0 + 150.0;
        let y = SCREEN_SIZE.1 / 2.0;
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

    pub fn move_position(&mut self, x: f32, y: f32) {
        self.square.x += x;
        self.square.y += y;
        self.update_corners();
    }

    pub fn update_corners(&mut self) {
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

pub struct Shape {
    pub shape: Mesh,
    pub corners: Vec<Point2<f32>>,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub max_velocity_y: f64,
    pub max_velocity_x: f64,
}

impl Shape {
    pub fn new(ctx: &mut Context) -> Shape {
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

        Shape { shape, corners, velocity_x: 0.0, velocity_y: 0.0, max_velocity_x: 15.0, max_velocity_y: 15.0 }
    }

    pub fn new1(ctx: &mut Context) -> Shape {
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

        Shape { shape, corners, velocity_x: 0.0, velocity_y: 0.0, max_velocity_x: 15.0, max_velocity_y: 15.0 }
    }

    pub fn new2(ctx: &mut Context) -> Shape {
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

        Shape { shape, corners, velocity_x: 0.0, velocity_y: 0.0, max_velocity_x: 15.0, max_velocity_y: 15.0 }
    }

    pub fn new3(ctx: &mut Context) -> Shape {
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
        let shape = Mesh::new_polygon(ctx, DrawMode::fill(), &corners, Color::WHITE).unwrap();

        Shape { shape, corners, velocity_x: 0.0, velocity_y: 0.0, max_velocity_x: 15.0, max_velocity_y: 15.0 }
    }

    pub fn move_position(&mut self, x: f32, y: f32, ctx: &mut Context) {
        if x != 0.0 || y != 0.0 {
            for corner in &mut self.corners {
                corner.x += x;
                corner.y += y;
            }

            self.shape = Mesh::new_polygon(ctx, DrawMode::fill(), &self.corners, Color::WHITE).unwrap();
        }
    }
}

impl Gravity for Shape {
    fn apply_gravity(&mut self, dt: f64) {
        self.velocity_y += Self::ACCELERATION * dt;
        if self.velocity_y > self.max_velocity_y {
            self.velocity_y = self.max_velocity_y;
        }
    }
}

pub struct Circle {
    pub circle: Mesh,
    pub center: Point2<f32>,
    pub radius: f32,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub max_velocity_x: f64,
    pub max_velocity_y: f64,
}

impl Gravity for Circle {
    fn apply_gravity(&mut self, dt: f64) {
        self.velocity_y += Self::ACCELERATION * dt;
        if self.velocity_y > self.max_velocity_y {
            self.velocity_y = self.max_velocity_y;
        }
    }
}

impl Circle {
    pub fn new(ctx: &mut Context) -> Circle {
        let center = Point2 { x: SCREEN_SIZE.0 / 2.0, y: SCREEN_SIZE.1 / 2.0};
        let radius = 50.0;
        let circle = Mesh::new_circle(ctx, DrawMode::fill(), center, radius, 0.01, Color::WHITE).unwrap();

        Circle { circle, center, radius, velocity_x: 0.0, velocity_y: 0.0, max_velocity_x: 15.0, max_velocity_y: 15.0 }
    }

    pub fn move_position(&mut self, x: f32, y: f32, ctx: &mut Context) {
        if x == 0.0 || y == 0.0 {
            self.center.x += x;
            self.center.y += y;

            self.circle = Mesh::new_circle(ctx, DrawMode::fill(), self.center, self.radius, 0.01, Color::WHITE).unwrap();
        }
    }
}