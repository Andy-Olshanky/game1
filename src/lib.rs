use ggez::{
    event::EventHandler,
    graphics::{self, Canvas, DrawParam, Image},
    mint::Point2,
    Context, GameResult,
};
use nalgebra::{vector, Vector2};
use rapier2d::{
    dynamics::{
        CCDSolver, ImpulseJointSet, IntegrationParameters, IslandManager, MultibodyJointSet, RigidBody, RigidBodyBuilder, RigidBodyHandle, RigidBodySet
    },
    geometry::{BroadPhase, ColliderBuilder, ColliderHandle, ColliderSet, NarrowPhase},
    pipeline::{PhysicsPipeline, QueryPipeline},
};
use std::f32::consts::PI;

pub const SCREEN_SIZE: (f32, f32) = (1000.0, 700.0);
const TARGET_FPS: f64 = 60.0;

pub struct Ball {
    ball: Image,
}

impl Ball {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let ball = Image::from_path(ctx, "\\ball.png")?;

        Ok(Self { ball })
    }
}

pub struct Floor {
    floor: Image,
}

impl Floor {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let floor = Image::from_path(ctx, "\\floor.png")?;

        Ok(Self { floor })
    }
}

pub struct World {
    gravity: Vector2<f32>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    query_pipeline: QueryPipeline,
    handle: RigidBodyHandle,
    floor_handle: RigidBodyHandle,
    floor_body: RigidBody,
    floor_collider_handle: ColliderHandle
}

impl World {
    pub fn new(
        rigid_body_set: RigidBodySet,
        collider_set: ColliderSet,
        handle: RigidBodyHandle,
        gravity: Vector2<f32>,
        floor_handle: RigidBodyHandle,
        floor_body: RigidBody,
        floor_collider_handle: ColliderHandle
    ) -> Self {
        // let gravity = vector![50.0, 0.0];
        // let gravity = vector![0.0, -50.0];
        let integration_parameters = IntegrationParameters::default();
        let physics_pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let query_pipeline = QueryPipeline::new();

        Self {
            gravity,
            integration_parameters,
            physics_pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            rigid_body_set,
            collider_set,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            query_pipeline,
            handle,
            floor_handle,
            floor_body,
            floor_collider_handle
        }
    }
}

pub struct GameState {
    ball: Ball,
    floor: Floor,
    world1: World,
    world2: World,
    world3: World,
    world4: World,
    floor_rotation: f32,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut rigid_body_set1 = RigidBodySet::new();
        let mut rigid_body_set2 = RigidBodySet::new();
        let mut rigid_body_set3 = RigidBodySet::new();
        let mut rigid_body_set4 = RigidBodySet::new();
        let mut collider_set1 = ColliderSet::new();
        let mut collider_set2 = ColliderSet::new();
        let mut collider_set3 = ColliderSet::new();
        let mut collider_set4 = ColliderSet::new();

        let floor = Floor::new(ctx).unwrap();

        let floor_body = RigidBodyBuilder::dynamic()
            .lock_translations()
            // .rotation(PI / 3.0)
            .angvel(PI / 6.0)
            .build();

        let floor_handle1 = rigid_body_set1.insert(floor_body.clone());
        let floor_handle2 = rigid_body_set2.insert(floor_body.clone());
        let floor_handle3 = rigid_body_set3.insert(floor_body.clone());
        let floor_handle4 = rigid_body_set4.insert(floor_body.clone());

        let collider = ColliderBuilder::cuboid(
            floor.floor.width() as f32 / 2.0,
            floor.floor.height() as f32 / 2.0,
        )
        .translation(vector![300.0, 500.0])
        // .rotation(PI / 3.0)
        .build();

        let floor_collider_handle1 = collider_set1.insert_with_parent(collider.clone(), floor_handle1, &mut rigid_body_set1);
        let floor_collider_handle2 = collider_set2.insert_with_parent(collider.clone(), floor_handle2, &mut rigid_body_set2);
        let floor_collider_handle3 = collider_set3.insert_with_parent(collider.clone(), floor_handle3, &mut rigid_body_set3);
        let floor_collider_handle4 = collider_set4.insert_with_parent(collider.clone(), floor_handle4, &mut rigid_body_set4);

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![310.0, 600.0])
            .build();
        let collider = ColliderBuilder::ball(16.0).restitution(0.0).build();
        let ball_body_handle1 = rigid_body_set1.insert(rigid_body);
        collider_set1.insert_with_parent(collider, ball_body_handle1, &mut rigid_body_set1);

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![600.0, 500.0])
            .build();
        let collider = ColliderBuilder::ball(16.0).restitution(0.0).build();
        let ball_body_handle2 = rigid_body_set2.insert(rigid_body);
        collider_set2.insert_with_parent(collider, ball_body_handle2, &mut rigid_body_set2);

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![310.0, 400.0])
            .build();
        let collider = ColliderBuilder::ball(16.0).restitution(0.0).build();
        let ball_body_handle3 = rigid_body_set3.insert(rigid_body);
        collider_set3.insert_with_parent(collider, ball_body_handle3, &mut rigid_body_set3);

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![150.0, 500.0])
            .build();
        let collider = ColliderBuilder::ball(16.0).restitution(0.0).build();
        let ball_body_handle4 = rigid_body_set4.insert(rigid_body);
        collider_set4.insert_with_parent(collider, ball_body_handle4, &mut rigid_body_set4);

        Ok(GameState {
            ball: Ball::new(ctx).unwrap(),
            floor,
            world1: World::new(
                rigid_body_set1,
                collider_set1,
                ball_body_handle1,
                vector![0.0, -100.0],
                floor_handle1,
                floor_body.clone(),
                floor_collider_handle1
            ),
            world2: World::new(
                rigid_body_set2,
                collider_set2,
                ball_body_handle2,
                vector![-100.0, 0.0],
                floor_handle2,
                floor_body.clone(),
                floor_collider_handle2
            ),
            world3: World::new(
                rigid_body_set3,
                collider_set3,
                ball_body_handle3,
                vector![0.0, 100.0],
                floor_handle3,
                floor_body.clone(),
                floor_collider_handle3
            ),
            world4: World::new(
                rigid_body_set4,
                collider_set4,
                ball_body_handle4,
                vector![100.0, 0.0],
                floor_handle4,
                floor_body.clone(),
                floor_collider_handle4
            ),
            floor_rotation: 0.0,
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let physics_hooks = ();
        let event_handler = ();
        while ctx.time.check_update_time(TARGET_FPS as u32) {
            self.world1.physics_pipeline.step(
                &self.world1.gravity,
                &self.world1.integration_parameters,
                &mut self.world1.island_manager,
                &mut self.world1.broad_phase,
                &mut self.world1.narrow_phase,
                &mut self.world1.rigid_body_set,
                &mut self.world1.collider_set,
                &mut self.world1.impulse_joint_set,
                &mut self.world1.multibody_joint_set,
                &mut self.world1.ccd_solver,
                Some(&mut self.world1.query_pipeline),
                &physics_hooks,
                &event_handler,
            );

            self.world2.physics_pipeline.step(
                &self.world2.gravity,
                &self.world2.integration_parameters,
                &mut self.world2.island_manager,
                &mut self.world2.broad_phase,
                &mut self.world2.narrow_phase,
                &mut self.world2.rigid_body_set,
                &mut self.world2.collider_set,
                &mut self.world2.impulse_joint_set,
                &mut self.world2.multibody_joint_set,
                &mut self.world2.ccd_solver,
                Some(&mut self.world2.query_pipeline),
                &physics_hooks,
                &event_handler,
            );

            self.world3.physics_pipeline.step(
                &self.world3.gravity,
                &self.world3.integration_parameters,
                &mut self.world3.island_manager,
                &mut self.world3.broad_phase,
                &mut self.world3.narrow_phase,
                &mut self.world3.rigid_body_set,
                &mut self.world3.collider_set,
                &mut self.world3.impulse_joint_set,
                &mut self.world3.multibody_joint_set,
                &mut self.world3.ccd_solver,
                Some(&mut self.world3.query_pipeline),
                &physics_hooks,
                &event_handler,
            );

            self.world4.physics_pipeline.step(
                &self.world4.gravity,
                &self.world4.integration_parameters,
                &mut self.world4.island_manager,
                &mut self.world4.broad_phase,
                &mut self.world4.narrow_phase,
                &mut self.world4.rigid_body_set,
                &mut self.world4.collider_set,
                &mut self.world4.impulse_joint_set,
                &mut self.world4.multibody_joint_set,
                &mut self.world4.ccd_solver,
                Some(&mut self.world4.query_pipeline),
                &physics_hooks,
                &event_handler,
            );

            self.floor_rotation += self.world1.floor_body.angvel() / TARGET_FPS as f32;

            // for rb in self.world1.rigid_body_set.iter() {
            //     println!("{:?}", rb.1);
            // }

            // println!("*******************************************");
            // let floor_body = &self.world1.rigid_body_set[self.world1.floor_handle];
            // println!("{}", floor_body.position().rotation.angle());
            // println!("*******************************************");
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::RED);

        canvas.draw(
            &self.floor.floor,
            DrawParam::default().dest(Point2 { x: 0.0, y: 0.0 }),
        );
        canvas.draw(
            &self.floor.floor,
            DrawParam::default().dest(Point2 {
                x: 0.0,
                y: SCREEN_SIZE.1 - self.floor.floor.height() as f32,
            }),
        );

        let floor_body = self.world1.rigid_body_set.get(self.world1.floor_handle).unwrap();
        let rotation = floor_body.rotation().angle();
        // println!("Rotation: {}", rotation);
        canvas.draw(
            &self.floor.floor,
            DrawParam::default()
                .dest(Point2 {
                    // x: 300.0 - self.floor.floor.width() as f32 + 66.0,
                    // y: SCREEN_SIZE.1 - 500.0 - self.floor.floor.height() as f32 - 3.0,
                    x: 300.0 + 16.0,
                    y: SCREEN_SIZE.1 - 500.0 - self.floor.floor.height() as f32 / 2.0 - 3.0
                })
                .offset(Point2 { x: 0.5, y: 0.5 })
                .rotation(-rotation),
        );

        let coords = &self.world1.rigid_body_set[self.world1.handle].translation();
        canvas.draw(
            &self.ball.ball,
            DrawParam::default().dest(Point2 {
                x: coords.x,
                y: SCREEN_SIZE.1 - coords.y - 32.0,
            }),
        );

        let coords = &self.world2.rigid_body_set[self.world2.handle].translation();
        canvas.draw(
            &self.ball.ball,
            DrawParam::default().dest(Point2 {
                x: coords.x,
                y: SCREEN_SIZE.1 - coords.y - 32.0,
            }),
        );

        let coords = &self.world3.rigid_body_set[self.world3.handle].translation();
        canvas.draw(
            &self.ball.ball,
            DrawParam::default().dest(Point2 {
                x: coords.x,
                y: SCREEN_SIZE.1 - coords.y - 32.0,
            }),
        );

        let coords = &self.world4.rigid_body_set[self.world4.handle].translation();
        canvas.draw(
            &self.ball.ball,
            DrawParam::default().dest(Point2 {
                x: coords.x,
                y: SCREEN_SIZE.1 - coords.y - 32.0,
            }),
        );

        canvas.finish(ctx)?;

        Ok(())
    }
}
