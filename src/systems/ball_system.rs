extern crate rand;

use amethyst::{
    core::{Float, Time, Transform},
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::pong::{BallComponent, ARENA_HEIGHT, ARENA_WIDTH, BALL_MAX_VELOCITY};
use nalgebra::Vector2;
use nalgebra::Vector3;

use rand::Rng;

const DEPENETRATION_FACTOR: f32 = 0.1;
const DEPENETRATION_CYCLES: i32 = 4;

pub struct BallSystem;

impl<'s> System<'s> for BallSystem {
    type SystemData = (
        Read<'s, Time>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, BallComponent>,
    );

    fn run(&mut self, (time, mut transforms, mut balls): Self::SystemData) {
        for (ball, transf) in (&mut balls, &mut transforms).join() {
            update_ball_position(&time, ball, transf);
        }
    }
}

fn update_ball_position(time: &Time, ball: &mut BallComponent, transf: &mut Transform) {
    perform_velocity_motion(time, ball, transf);
    let collision_normal = perform_sphere_depenetration(ball, transf);
    perform_bouncing(collision_normal, ball);
}

fn perform_velocity_motion(time: &Time, ball: &BallComponent, transf: &mut Transform) {
    let &position = transf.translation();
    let mut position = Vector2::new(position.x.as_f32(), position.y.as_f32());

    position += ball.get_velocity() * time.delta_seconds();

    transf.set_translation(Vector3::new(
        Float::from(position.x),
        Float::from(position.y),
        Float::from(0.0),
    ));
}

fn perform_sphere_depenetration(
    ball: &BallComponent,
    transf: &mut Transform,
) -> Option<Vector2<f32>> {
    let &position = transf.translation();
    let mut position = Vector2::new(position.x.as_f32(), position.y.as_f32());

    let mut collision_count: i32 = 0;
    let mut collision_normal = Vector2::new(0.0, 0.0);

    for _i in 0..DEPENETRATION_CYCLES {
        // Check collision against Left margin
        let res = detect_quad_collision(
            Vector2::new(ARENA_WIDTH, ARENA_HEIGHT),
            &position,
            ball.get_radius(),
        );

        if res.is_some() {
            collision_count += 1;
            collision_normal += res.unwrap().1;
        }
        perform_depenetration(res, &mut position, &DEPENETRATION_FACTOR);
    }

    transf.set_translation(Vector3::new(
        Float::from(position.x),
        Float::from(position.y),
        Float::from(0.0),
    ));

    if collision_count != 0 {
        Some(collision_normal.normalize())
    } else {
        None
    }
}

type CollisionResult = Option<(f32, Vector2<f32>)>;

// This function returns a touple with the penetration distance, the collision normal and the
// collision world position
/*fn detect_plane_collision(
    plane_dist: f32,
    plane_normal: Vector2<f32>,
    sphere_pos: &Vector2<f32>,
    sphere_radius: f32,
) -> CollisionResult {
    let dot = plane_normal.dot(sphere_pos);
    let is_colliding = dot > plane_dist - sphere_radius;

    if is_colliding {
        let on_sphere_distance = (dot - plane_dist).abs() - sphere_radius;
        let penetration = on_sphere_distance * -1.0;
        let collision_pos = sphere_pos + plane_normal * (penetration - sphere_radius);
        Some((penetration, plane_normal, collision_pos))
    } else {
        None
    }
}*/

fn detect_quad_collision(
    margins: Vector2<f32>,
    sphere_pos: &Vector2<f32>,
    sphere_radius: f32,
) -> CollisionResult {
    // Check Left
    let margin = 0.0;
    let penetration = sphere_pos.x - (margin + sphere_radius);
    if penetration < 0.0 {
        return Some((penetration*-1.0, Vector2::new(1.0, 0.0)));
    }

    // Check Right
    let margin = margins.x;
    let penetration = (margin - sphere_radius) - sphere_pos.x;
    if penetration < 0.0 {
        return Some((penetration*-1.0, Vector2::new(-1.0, 0.0)));
    }

    // Check Bottom
    let margin = 0.0;
    let penetration = sphere_pos.y - (margin + sphere_radius);
    if penetration < 0.0 {
        return Some((penetration*-1.0, Vector2::new(0.0, 1.0)));
    }

    // Check Top
    let margin = margins.y;
    let penetration = (margin - sphere_radius) - sphere_pos.y;
    if penetration < 0.0 {
        return Some((penetration*-1.0, Vector2::new(0.0, -1.0)));
    }
    None
}

fn perform_depenetration(res: CollisionResult, pos: &mut Vector2<f32>, factor: &f32) {
    if let Some(res) = res {
        *pos += res.1 * (res.0 * factor);
    }
}

fn perform_bouncing(collision_normal: Option<Vector2<f32>>, ball: &mut BallComponent) {
    if let Some(collision_normal) = collision_normal {
        let v = (collision_normal * ball.get_velocity().dot(&collision_normal)) * 2.0 
            - ball.get_velocity();
        ball.set_velocity(v.normalize() * -BALL_MAX_VELOCITY);
        //ball.set_velocity((collision_normal + Vector2::new(rand::thread_rng().gen_range(0.0, 1.1), rand::thread_rng().gen_range(0.0, 1.1))).normalize() * BALL_MAX_VELOCITY);
    }
}
