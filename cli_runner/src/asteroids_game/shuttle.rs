use specs_derive::Component;
use specs::prelude::*;
use crate::asteroids_game::physics::RigidBody;
use std::time::{Duration, Instant};
use crate::asteroids_game::renderer::{Renderable, OutlineFactory};
use euclid::vec2;
use crate::asteroids_game::pose::Pose;
use lyon::geom::euclid::Angle;
use crate::asteroids_game::collider::{Collider, ColliderType};
use crate::asteroids_game::collision_manager::player_projectile_collision_handler;
use lyon_path::Path;
use std::borrow::Borrow;

#[derive(Component)]
pub struct Score{
    score: u32,
    last_score_update: Instant,
}

impl Score {
    pub fn new() -> Score{
        Score{ score: 0, last_score_update: Instant::now()}
    }
    pub fn get_score(&self) -> u32 {
        self.score
    }
    pub fn add_score(&mut self, gain: u32) {
        self.score += gain;
    }
}

pub struct ScoreUpdater;

impl<'a> System<'a> for ScoreUpdater {
    type SystemData = WriteStorage<'a, Score>;

    fn run(&mut self, mut scores : Self::SystemData) {
        for score in (&mut scores).join() {
            let now = Instant::now();
            let score_gain_interval = Duration::from_secs(1);
            if (now-score.last_score_update) > score_gain_interval {
                score.last_score_update += score_gain_interval;
                score.score += 1;
            }
        }
    }
}

#[derive(Component)]
pub struct Shuttle{
    lives: u32,
    pub accelerating: bool,
    pub shield: bool,
    auto_shield_start_time: Instant,
    auto_shield_duration: Duration,
}

impl Shuttle {
    pub fn new() -> Shuttle{
        Shuttle {
            lives: 3,
            accelerating: false,
            shield: true,
            auto_shield_start_time: Instant::now(),
            auto_shield_duration: Duration::from_secs(5),

        }
    }
    pub fn handle_collsion_with_enemy(&mut self, own_entity: Entity, world: &World) {
        if !self.shield {
        if self.lives > 1 {
            self.lives -= 1;
            self.start_auto_shield(Instant::now().borrow());
            let mut poses = world.write_storage::<Pose>();
            let mut shuttle_pose = poses.get_mut(own_entity).unwrap();
            let mut rigidbodies = world.write_storage::<RigidBody>();
            let shuttle_rigidbody = rigidbodies.get_mut(own_entity).unwrap();
            shuttle_pose.rot = Angle::zero();
            shuttle_pose.x = 0.0;
            shuttle_pose.y = 0.0;
            shuttle_rigidbody.reset_motion();
        } else {
            // Player died. #Sad
            self.lives = 0;
        }
        }
    }

    pub fn is_dead(&self) -> bool {
        self.lives == 0
    }
    pub fn get_lives(&self) -> u32 {
        self.lives
    }

    pub fn start_auto_shield(&mut self, now: &Instant) {
        self.shield = true;
        self.auto_shield_start_time = *now;

    }

    pub fn update_auto_shield(&mut self, now: &Instant) {
        if self.shield && self.auto_shield_duration < (*now - self.auto_shield_start_time) {
                 self.shield = false;
            }
    }

    pub fn get_shuttle_auto_shield_outline(&self, now: &Instant) -> Path{
        let blink_cycle_s:f32 = 0.5;
        if let Some(auto_shield_remaining_time) = self.auto_shield_duration.checked_sub(
        *now - self.auto_shield_start_time) {
            if auto_shield_remaining_time < Duration::from_secs(2) {
                if auto_shield_remaining_time.as_secs_f32() % blink_cycle_s < (blink_cycle_s/2.0){
                    OutlineFactory::shuttle_shield()
                } else {
                    Path::new()
                }
            } else {
                OutlineFactory::shuttle_shield()
            }
        } else {
            Path::new()
        }
    }
}
pub struct ShuttleUpdater;

impl<'a> System<'a> for ShuttleUpdater {
    type SystemData = (WriteStorage<'a, Shuttle>, WriteStorage<'a, Renderable>);

    fn run(&mut self, (mut shuttles, mut renderables) : Self::SystemData) {
        for (shuttle, renderable) in (&mut shuttles, &mut renderables).join() {
        let now = Instant::now();
        shuttle.update_auto_shield(now.borrow());

        if shuttle.is_dead() {
            renderable.outline = Path::new();
        } else {
            renderable.outline = OutlineFactory::shuttle_outline();
                if shuttle.shield {
                let shield_outline = shuttle.get_shuttle_auto_shield_outline(now.borrow());
                    renderable.outline = renderable.outline.merge(shield_outline.borrow());
                }
            }
        }
    }
}

#[derive(Component)]
pub struct Gun{
    pub firing: bool,
    rate_of_fire: Duration,
    last_fire_time: Instant,
}

#[derive(Component)]
pub struct Hierarchy{
    pub parent: Entity,
}

impl Gun {
    pub fn new() -> Gun{
        Gun {
            firing: false,
            rate_of_fire: Duration::new(0, 500000000),
            last_fire_time: Instant::now(),
        }
    }
}

pub struct ShootingHandler;

impl<'a> System<'a> for ShootingHandler {
    type SystemData = (ReadStorage<'a, RigidBody>,
                       ReadStorage<'a, Pose>,
                       WriteStorage<'a, Gun>,
                       Read<'a, LazyUpdate>,
                       Entities<'a>

    );


    fn run(&mut self, (rigidbody, pose, mut gun, updater, entities) : Self::SystemData) {
        for (rigidbody, pose, gun, entity) in (&rigidbody, &pose, &mut gun, &entities).join() {
            if gun.firing {
                let elasped_time = gun.last_fire_time.elapsed();
                if gun.rate_of_fire < elasped_time{
                    gun.last_fire_time = Instant::now();
                    let (x,y) = pose.rot.sin_cos();
                    let mut projectile_rigidbody = rigidbody.clone();
                    projectile_rigidbody.velocity += vec2(x,y)*0.03;
                    projectile_rigidbody.angular_velocity = Angle::zero();
                    projectile_rigidbody.looping = false;
                    let projectile = entities.create();
                    updater.insert(projectile, projectile_rigidbody.clone());
                    updater.insert(projectile, Renderable{outline:OutlineFactory::projectile_outline()});
                    updater.insert(projectile, pose.clone());
                    updater.insert(projectile, Collider{
                        collider_type: ColliderType::Projectile,
                        collider_outline: OutlineFactory::projectile_collider_outline(),
                        collision_handler: player_projectile_collision_handler
                    });
                    updater.insert(projectile, Hierarchy{parent: entity});
                }
            }
        }
    }
}



