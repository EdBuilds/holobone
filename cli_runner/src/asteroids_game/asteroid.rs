use specs_derive::Component;
use specs::prelude::*;
use lyon::geom::euclid::Angle;
use euclid::default::Vector2D;
use rand::rngs::ThreadRng;
use crate::asteroids_game::physics::RigidBody;
use crate::asteroids_game::collider::{Collider, ColliderType};
use crate::asteroids_game::collision_manager::asteroid_collision_handler;
use crate::asteroids_game::pose::Pose;
use crate::asteroids_game::renderer::{OutlineFactory, Renderable};
use rand::Rng;

#[derive(Component, Clone)]
pub(crate) struct Asteroid {
    pub(crate) level: u32
}

pub fn asterod_builder(entity:EntityBuilder, level: u32, x: f32, y: f32, velocity: Vector2D<f32>, mut rng:ThreadRng) -> EntityBuilder{
    let double_outline = OutlineFactory::asteroid_outline(level);
    entity.with(Pose{
        x: x,
        y: y,
        rot: Angle::zero(),
    })
        .with(RigidBody {
            velocity: velocity,
            angular_velocity: Angle::radians(rng.gen_range(-0.2,0.2)),
            last_update: None,
            looping: true,
        })
        .with(Renderable{
            outline: double_outline.clone()
        })
        .with(Collider {
            collider_type: ColliderType::Enemy,
            collider_outline: double_outline,
            collision_handler: asteroid_collision_handler
        })
        .with(Asteroid{level})
}
