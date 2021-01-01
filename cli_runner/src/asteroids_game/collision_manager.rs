use crate::asteroids_game::asteroid::{asterod_builder, Asteroid};
use crate::asteroids_game::collider::{
    Collider, ColliderType, CollisionHandler, CollisionHandlerError,
};
use crate::asteroids_game::physics::RigidBody;
use crate::asteroids_game::pose::Pose;
use crate::asteroids_game::shuttle::{Hierarchy, Score, Shuttle};
use lyon::geom::euclid::{Rotation2D, Translation2D};
use lyon::lyon_algorithms::hit_test::hit_test_path;
use lyon::lyon_algorithms::path::FillRule;
use rand::{thread_rng, Rng};
use specs::prelude::*;
use specs::{Entities, Entity, LazyUpdate, Read, ReadStorage, System, World, WorldExt};

pub struct CollisionManager;

impl<'a> System<'a> for CollisionManager {
    type SystemData = (
        ReadStorage<'a, Pose>,
        ReadStorage<'a, Collider>,
        Read<'a, LazyUpdate>,
        Entities<'a>,
    );

    fn run(&mut self, (poses, colliders, updater, entities): Self::SystemData) {
        type CollisionDescriptor = ([CollisionHandler; 2], [Entity; 2]);
        let mut collisions_to_handle = Vec::<CollisionDescriptor>::new();

        for (pose, collider, entity) in (&poses, &colliders, &entities).join() {
            let rotation = Rotation2D::new(-pose.rot);
            let translation = Translation2D::new(pose.x, pose.y);

            for (other_pose, other_collider, other_entity) in (&poses, &colliders, &entities).join()
            {
                let other_rotation = Rotation2D::new(-other_pose.rot);
                let other_translation = Translation2D::new(other_pose.x, other_pose.y);
                let transformed_other_collider_outline = other_collider
                    .collider_outline
                    .transformed(&other_rotation)
                    .transformed(&other_translation);

                let mut collision = false;
                for evt in transformed_other_collider_outline.iter() {
                    // This needs some love
                    let test_point = evt.from();
                    let transformed_collider_outline = collider
                        .collider_outline
                        .transformed(&rotation)
                        .transformed(&translation);
                    if hit_test_path(
                        &test_point,
                        transformed_collider_outline.iter(),
                        FillRule::NonZero,
                        0.01,
                    ) {
                        collision = true;
                        break;
                    }
                }
                if collision {
                    collisions_to_handle.push((
                        [collider.collision_handler, other_collider.collision_handler],
                        [entity, other_entity],
                    ));
                }
            }
        }
        updater.exec_mut(move |world| {
            for (collision_handlers, collision_entities) in collisions_to_handle {
                if collision_entities[0] != collision_entities[1] {
                    let _ = (collision_handlers[0])(
                        collision_entities[0],
                        collision_entities[1],
                        world,
                    );
                    let _ = (collision_handlers[1])(
                        collision_entities[1],
                        collision_entities[0],
                        world,
                    );
                }
            }
        });
    }
}

// TODO: find a way to have an unused placeholder function withouth warnings
//fn placeholder_collision_handler(current_entity: Entity, other_entity: Entity, world:&mut World) -> Result<(), CollisionHandlerError>{Err(CollisionHandlerError::NotImplemented)}

pub fn asteroid_collision_handler(
    current_entity: Entity,
    other_entity: Entity,
    world: &mut World,
) -> Result<(), CollisionHandlerError> {
    let other_collider_type = world
        .read_storage::<Collider>()
        .get(other_entity)
        .ok_or(CollisionHandlerError::MissingComponent)?
        .collider_type;
    let next_asteroid_level = world
        .read_storage::<Asteroid>()
        .get(current_entity)
        .ok_or(CollisionHandlerError::MissingComponent)?
        .level
        - 1;
    let asteroid_pos = world
        .read_storage::<Pose>()
        .get(current_entity)
        .ok_or(CollisionHandlerError::MissingComponent)?
        .clone();
    let asteroid_rigidbody = world
        .read_storage::<RigidBody>()
        .get(current_entity)
        .ok_or(CollisionHandlerError::MissingComponent)?
        .clone();
    let mut rng = thread_rng();
    if let ColliderType::Projectile = other_collider_type {
        if next_asteroid_level > 0 {
            //TODO: base perturbation on the input angle of the projectile
            let vel_perturbation =
                euclid::vec2(rng.gen_range(-0.005, 0.005), rng.gen_range(-0.005, 0.005));
            let new_velocities = [
                asteroid_rigidbody.velocity + vel_perturbation,
                asteroid_rigidbody.velocity - vel_perturbation,
            ];
            for new_velocity in new_velocities.iter() {
                //TODO: fix random number generator
                asterod_builder(
                    world.create_entity(),
                    next_asteroid_level,
                    asteroid_pos.x,
                    asteroid_pos.y,
                    *new_velocity,
                    rand::thread_rng(),
                )
                .build();
            }
        }
        let _result = world.delete_entity(current_entity);
    }
    Ok(())
}

pub fn player_collision_handler(
    current_entity: Entity,
    other_entity: Entity,
    world: &mut World,
) -> Result<(), CollisionHandlerError> {
    let other_collider_type = world
        .read_storage::<Collider>()
        .get(other_entity)
        .ok_or(CollisionHandlerError::MissingComponent)?
        .collider_type;
    let mut shuttles = world.write_storage::<Shuttle>();
    let shuttle = shuttles
        .get_mut(current_entity)
        .ok_or(CollisionHandlerError::MissingComponent)?;
    if let ColliderType::Enemy = other_collider_type {
        shuttle.handle_collsion_with_enemy(current_entity, world);
    }
    Ok(())
}

pub fn player_projectile_collision_handler(
    current_entity: Entity,
    other_entity: Entity,
    world: &mut World,
) -> Result<(), CollisionHandlerError> {
    let colliders = world.read_storage::<Collider>();
    let hierarchies = world.read_storage::<Hierarchy>();
    let mut scores = world.write_storage::<Score>();
    let mut maybe_other_collider_type = None;
    if let Some(collider) = colliders.get(other_entity) {
        maybe_other_collider_type = Some(collider.collider_type);
    }
    let entities = world.entities();
    let mut score_gain: u32 = 0;
    match maybe_other_collider_type {
        Some(other_collider_type) => match other_collider_type {
            ColliderType::Player => {}
            ColliderType::Projectile => {}
            _ => {
                let _result = entities.delete(current_entity);
                score_gain = 5;
            }
        },
        _ => {
            let _result = entities.delete(current_entity);
            score_gain = 5;
        }
    }
    if let Some(hierarchy) = hierarchies.get(current_entity) {
        if let Some(shooter_score) = scores.get_mut(hierarchy.parent) {
            shooter_score.add_score(score_gain);
        }
    }
    Ok(())
}
