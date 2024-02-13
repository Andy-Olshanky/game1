use ggez::{
    event::EventHandler,
    graphics::{self, Canvas, DrawParam, Image},
    mint::Point2,
    Context, GameResult,
};
use nalgebra::{vector, Vector2};
use rapier2d::{
    dynamics::{
        CCDSolver, ImpulseJointSet, IntegrationParameters, IslandManager, MultibodyJointSet, RigidBodyBuilder, RigidBodyHandle, RigidBodySet
    },
    geometry::{BroadPhase, ColliderBuilder, ColliderSet, NarrowPhase},
    pipeline::{PhysicsPipeline, QueryPipeline},
};

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
}

impl World {
    pub fn new(
        rigid_body_set: RigidBodySet,
        collider_set: ColliderSet,
        handle: RigidBodyHandle,
    ) -> Self {
        let gravity = vector![0.0, -9.8];
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
        }
    }
}

pub struct GameState {
    ball: Ball,
    world: World,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();

        let collider = ColliderBuilder::cuboid(100.0, 1.0).build();
        collider_set.insert(collider);

        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![0.0, 250.0])
            .build();
        let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
        let ball_body_handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

        Ok(GameState {
            ball: Ball::new(ctx).unwrap(),
            world: World::new(rigid_body_set, collider_set, ball_body_handle),
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let physics_hooks = ();
        let event_handler = ();
        while ctx.time.check_update_time(TARGET_FPS as u32) {
            self.world.physics_pipeline.step(
                &self.world.gravity,
                &self.world.integration_parameters,
                &mut self.world.island_manager,
                &mut self.world.broad_phase,
                &mut self.world.narrow_phase,
                &mut self.world.rigid_body_set,
                &mut self.world.collider_set,
                &mut self.world.impulse_joint_set,
                &mut self.world.multibody_joint_set,
                &mut self.world.ccd_solver,
                Some(&mut self.world.query_pipeline),
                &physics_hooks,
                &event_handler,
            );
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::BLACK);

        let coords = &self.world.rigid_body_set[self.world.handle].translation();
        canvas.draw(&self.ball.ball, DrawParam::default().dest(Point2 { x: coords.x, y: SCREEN_SIZE.1 - coords.y - 32.0 }));

        canvas.finish(ctx)?;

        Ok(())
    }
}
