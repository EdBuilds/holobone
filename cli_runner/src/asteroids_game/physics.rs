use euclid::default::Vector2D;
use lyon::geom::euclid::Angle;
use specs::{System, ReadStorage, Entities, WriteStorage};
use crate::asteroids_game::pose::Pose;
use specs_derive::Component;
use specs::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct RigidBody {
    pub velocity: Vector2D<f32>,
    pub angular_velocity: Angle<f32>,
    pub last_update: Option<f32>,
    pub looping: bool,
}
impl RigidBody{
    pub fn reset_motion(&mut self) {
        self.velocity.x = 0.0;
        self.velocity.y = 0.0;
        self.angular_velocity.radians = 0.0;
    }
}

pub struct MovementSimulator;

impl<'a> System<'a> for MovementSimulator {
    type SystemData = (ReadStorage<'a, RigidBody>,
                       WriteStorage<'a, Pose>,
                       Entities<'a>);

    fn run(&mut self, (rigidbody, mut pose, entities) : Self::SystemData) {
        for (rigidbody, pose, entity) in (&rigidbody, &mut pose, &entities).join() {
            pose.x += rigidbody.velocity.x;
            pose.y += rigidbody.velocity.y;
            pose.rot += rigidbody.angular_velocity;
            if rigidbody.looping {
                while pose.x > 1.0{
                    pose.x -= 2.0;
                }
                while pose.y > 1.0{
                    pose.y -= 2.0;
                }
                while pose.x < -1.0{
                    pose.x += 2.0;
                }
                while pose.y < -1.0{
                    pose.y += 2.0;
                }
            } else {
                if pose.x < -1.0 || pose.x > 1.0 ||
                    pose.y < -1.0 || pose.y > 1.0 {
                    let _result = entities.delete(entity);
                }
            }
        }
    }
}
