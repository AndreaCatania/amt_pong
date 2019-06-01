extern crate nalgebra;

use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{
    camera::{Camera, Projection},
    formats::texture::ImageFormat,
    loaders,
    sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle},
    types::TextureData,
    Texture,
};

use nalgebra::Vector2;

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;
pub const PADDLE_WIDTH: f32 = 4.0;
pub const PADDLE_HEIGHT: f32 = 32.0;
pub const PADDLE_VELOCITY: f32 = 50.0; // unit per sec
pub const BALL_MAX_VELOCITY: f32 = 200.0; // Unit per sec
pub const BALL_RADIUS: f32 = 2.0;

pub struct Pong {}

impl SimpleState for Pong {
    /// --- Notifications ---
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        /// This function is called by the engine
        Pong::init(data.world);

        Pong::initialize_camera(data.world);

        let sprite_sheet = Pong::load_sprite(data.world);
        Pong::initialize_paddle(data.world, &Side::Left, &sprite_sheet);
        Pong::initialize_paddle(data.world, &Side::Right, &sprite_sheet);
        Pong::initialize_ball(data.world, &sprite_sheet);

        println!("Pong game start is executed");
    }
}

impl Pong {
    fn init(world: &mut World) {
        // Register component storage for the Paddle entity
        world.register::<Paddle>();
    }

    fn initialize_camera(world: &mut World) {
        let mut transform = Transform::default();
        transform.set_translation_xyz(0.0, 0.0, 1.0); // Move the camera 1 unit away from the arena board

        world
            .create_entity()
            .with(Camera::from(Projection::orthographic(
                0.0,
                ARENA_WIDTH,
                -ARENA_HEIGHT,
                0.0,
                0.1,
                2000.0,
            )))
            .with(transform)
            .build();

        println!("Camera created");
    }

    fn initialize_paddle(world: &mut World, side: &Side, sprite_sheet: &SpriteSheetHandle) {
        // Create the paddle component
        let paddle = Paddle::new(&side);

        // Create the transform component
        let mut transform = Transform::default();
        transform.set_translation_xyz(
            match side {
                Side::Left => PADDLE_WIDTH,
                Side::Right => ARENA_WIDTH - PADDLE_WIDTH,
            },
            ARENA_HEIGHT / 2.0,
            0.0,
        );

        // Create the render compoennt
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 0, // The paddle is at position 0
        };

        // Create the entity
        world
            .create_entity()
            .with(paddle)
            .with(transform)
            .with(sprite_render)
            .build();
    }

    fn initialize_ball(world: &mut World, sprite_sheet: &SpriteSheetHandle) {
        // Create the entity components

        let mut ball = BallComponent::new();
        ball.set_velocity(Vector2::new(0.5, -1.0).normalize() * BALL_MAX_VELOCITY);

        let mut transform = Transform::default();
        transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);

        let sprite = SpriteRender {
            sprite_sheet: sprite_sheet.clone(),
            sprite_number: 1,
        };

        // Create the Entity
        world
            .create_entity()
            .with(ball)
            .with(transform)
            .with(sprite)
            .build();
    }

    fn load_sprite(world: &mut World) -> SpriteSheetHandle {
        // This function is resopnsible for loading the sprite sheet
        // and is composed by two section.

        // Section 1. Load the texture
        // This handle is a cloneable reference to the texture
        let texture_handle = {
            let loader = world.read_resource::<Loader>();
            let storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                "textures/pong_spritesheet.png", // Texture path
                ImageFormat::default(),
                (),
                &storage,
            )
        };

        // Section 2. Load and returns the sprite sheet
        let loader = world.read_resource::<Loader>();
        let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "textures/pong_spritesheet.ron", // Here we load the associated ron file
            SpriteSheetFormat(texture_handle), // We pass it the texture we want it to use
            (),
            &sprite_sheet_store,
        )
    }
}

pub enum Side {
    Left,
    Right,
}

// TODO please rename this to PaddleComponent
pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

impl Paddle {
    pub fn new(side: &Side) -> Paddle {
        Paddle {
            side: match side {
                Side::Left => Side::Left,
                Side::Right => Side::Right,
            },
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

pub struct BallComponent {
    velocity: Vector2<f32>,
    radius: f32,
}

impl Component for BallComponent {
    type Storage = DenseVecStorage<Self>;
}

impl BallComponent {
    pub fn new() -> BallComponent {
        BallComponent {
            velocity: Vector2::new(0.0, 0.0),
            radius: BALL_RADIUS,
        }
    }

    pub fn get_velocity(&self) -> &Vector2<f32> {
        &self.velocity
    }

    pub fn set_velocity(&mut self, velocity: Vector2<f32>) {
        self.velocity = velocity;
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }
}
