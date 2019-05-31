

use amethyst::{
    core::{Transform, Float, Time},
    ecs::{ Join, Read, ReadStorage, System, WriteStorage, },
    input::{StringBindings, InputHandler}
};

extern crate nalgebra as na;
use na::Vector3;

use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT, PADDLE_VELOCITY};

pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem{
    type SystemData = (
        Read<'s, Time>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Paddle>,
        Read<'s, InputHandler<StringBindings>>,
    );


    fn run(&mut self, (time, mut transforms, paddles, input): Self::SystemData){
        for (paddle, transform) in (&paddles, &mut transforms).join(){

            // Read input force
            let input_magnitude = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };

            // Move
            if input_magnitude.is_none() {
                return;
            }

            let input_magnitude = input_magnitude.unwrap() as f32;
            let movement = PADDLE_VELOCITY * input_magnitude * time.delta_seconds();

            transform.set_translation_y(
                Float::from(
                    (transform.translation().y.as_f32() + movement)
                    .max(0.0 + PADDLE_HEIGHT * 0.5)
                    .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                )
            );
        }

        println!("FPS: {}", 1.0/time.delta_seconds());
    }
}