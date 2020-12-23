use lyon_path::{Path, PositionStore};
use crate::renderer::Renderer;
use crate::renderer::RenderingError;
use gilrs::{Gilrs, Button, Event, GamepadId, Axis};
use std::borrow::{Borrow, BorrowMut};
use lyon_path::math::{point};
use std::marker::PhantomData;
use gilrs::ev::EventType::ButtonReleased;
use crate::text_renderer::{TextRenderer, TextAlignment};
use lyon::lyon_svg::parser::AttributeId::TextRendering;
use lyon::geom::euclid::{Translation2D, Rotation2D, Angle, Rect};
use lyon::path::FillRule;
use lyon::path::iterator::*;
use rltk::{Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use std::time::{Duration, Instant};
use specs_derive::Component;
use euclid::default::Vector2D;
use euclid::vec2;
use lyon::geom::euclid::{Point2D, Size2D};
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;
use crate::asteroids_game::collision_manager::*;
use crate::asteroids_game::pose::Pose;
use crate::asteroids_game::asteroid::{Asteroid, asterod_builder};
use crate::asteroids_game::physics::{RigidBody, MovementSimulator};
use crate::asteroids_game::collider::{ColliderType, Collider};
use crate::asteroids_game::renderer::{OutlineFactory, Renderable, render_entities};
use crate::asteroids_game::shuttle::{Shuttle, Gun, ShootingHandler, ShuttleUpdater, Score, ScoreUpdater, Hierarchy};

enum AsteroidsGameState {
    WaitForStart,
    Running,
    GameEnded,
}
struct UITelemetry{
    pub lives:u32,
    pub score:u32,
}
pub struct AsteroidsRenderer<'a, 'b> {
    gilrs: gilrs::Gilrs,
    active_gamepad : Option<gilrs::GamepadId>,
    game_state: AsteroidsGameState,
    ecs: World,
    dispatcher: Dispatcher<'a, 'b>,
    rng: ThreadRng,
    last_asteroid_spawn_time: Instant,
    asteroid_spawn_rate: Duration,
    text_renderer: &'a TextRenderer,
    player_entity: Option<Entity>,
    final_score: u32,
}
impl <'a, 'b>AsteroidsRenderer<'a, 'b> {

    pub fn new(text_renderer: &'a TextRenderer) -> Result<AsteroidsRenderer<'a, 'b>, RenderingError> {
        let mut rng = rand::thread_rng();
        let mut gilrs = Gilrs::new();
        let mut ecs = World::new();
        let asteroid_spawn_rate = Duration::new(5,0);
        let last_asteroid_spawn_time = Instant::now();
        ecs.register::<Pose>();
        ecs.register::<RigidBody>();
        ecs.register::<Renderable>();
        ecs.register::<Asteroid>();
        ecs.register::<Collider>();
        ecs.register::<Shuttle>();
        ecs.register::<Score>();
        ecs.register::<Hierarchy>();

        let mut dispatcher = DispatcherBuilder::new()
            .with(MovementSimulator, "MovementSimulator", &[])
            .with(ShootingHandler, "ShootingHandler", &[])
            .with(CollisionManager, "CollisionManager", &[])
            .with(ShuttleUpdater, "ShuttleUpdater", &[])
            .with(ScoreUpdater, "ScoreUpdater", &[])
            .build();
        dispatcher.setup(&mut ecs);

        match gilrs {
            Ok(gilrs) =>
                {Result::Ok(AsteroidsRenderer{
                   gilrs,
                   active_gamepad: None,
                   game_state: AsteroidsGameState::WaitForStart,
                   ecs,
                   dispatcher,
                   rng,
                   last_asteroid_spawn_time,
                   asteroid_spawn_rate,
                   text_renderer,
                   player_entity: None,
                   final_score: 0,
                })},
            Err(_) => {Result::Err(RenderingError::GamepadError)}
        }

    }

    fn run_systems(&mut self) {
        let mut ms = MovementSimulator{};
        ms.run_now(&self.ecs);
        self.ecs.maintain();
    }


    fn shuttle_update(&mut self, mut x_input: f32, mut y_input: f32, mut rot_input: f32, firing: bool) -> Option<UITelemetry>{
        // Deadzone around 0 for the input
        if x_input.abs() < 0.1 {
            x_input = 0.0;
        }
        if y_input.abs() < 0.1 {
            y_input = 0.0;
        }
        if rot_input.abs() < 0.1 {
            rot_input = 0.0;
        }
        match self.player_entity {
        Some(player_entity) => {
            let mut rigidbodies = self.ecs.write_storage::<RigidBody>();
            let mut Poses = self.ecs.write_storage::<Pose>();
            let mut shuttles = self.ecs.write_storage::<Shuttle>();
            let mut guns = self.ecs.write_storage::<Gun>();
            let mut scores = self.ecs.read_storage::<Score>();
            let rigidbody = rigidbodies.get_mut(player_entity)?;
            let pose = Poses.get_mut(player_entity)?;
            let mut shuttle = shuttles.get_mut(player_entity)?;
            let mut gun = guns.get_mut(player_entity)?;
            let score = scores.get(player_entity)?;
            let acc_scaler = 0.0005 as f32;
            let vel_scaler = 0.01 as f32;
            let rot_scaler = 0.2 as f32;
            let debug_movement = false;

            if debug_movement{
                rigidbody.velocity = vec2(0.0,0.0);
                pose.y +=y_input*vel_scaler;
                pose.x -=x_input*vel_scaler;
            }
            else {
                let (x,y) = pose.rot.sin_cos();
                rigidbody.velocity +=vec2(x,y)*y_input*acc_scaler;
            }
            gun.firing = firing;
            pose.rot -=Angle{radians: (rot_input*rot_scaler)};
            if shuttle.is_dead() {
                self.game_state = AsteroidsGameState::GameEnded;
            }
            Some(UITelemetry{ lives: shuttle.get_lives(), score: score.get_score() })
        }
            _ => {
                // Player update called when the player is not yet created
                None
            }
        }
    }

    fn spawn_asteroids(&mut self) {
        let elasped_time = self.last_asteroid_spawn_time.elapsed();
        if self.asteroid_spawn_rate < elasped_time{
            self.last_asteroid_spawn_time = Instant::now();
            asterod_builder(self.ecs.create_entity(), self.rng.gen_range(1,3), self.rng.gen_range(0.0,1.0),self.rng.gen_range(0.0,1.0),
            vec2(self.rng.gen_range(-0.005,0.005),self.rng.gen_range(-0.005,0.005)), self.rng)
            .build();
        }
    }

    fn render_ui(&self, telemetry:UITelemetry) -> Result<Path, RenderingError> {
        let score_text_box = Rect{ origin: Point2D {
            x: -0.95,
            y: -0.95 + 0.1,
            _unit: PhantomData
        }, size: Size2D {
            width: 0.95,
            height: 0.1,
            _unit: PhantomData
        } };
        let lives_text_box = Rect{ origin: Point2D {
            x: 0.0,
            y: -0.95 + 0.1,
            _unit: PhantomData
        }, size: Size2D {
            width: 0.95,
            height: 0.1,
            _unit: PhantomData
        } };
        let score_ui_path = self.text_renderer.print(telemetry.score.to_string().as_ref(), &score_text_box, (TextAlignment::Left))?;
        let lives_ui_path = self.text_renderer.print(telemetry.lives.to_string().as_ref(), &lives_text_box, (TextAlignment::Right))?;
        Ok(score_ui_path.merge(lives_ui_path.borrow()))
    }
}


impl Renderer for AsteroidsRenderer<'_, '_> {
    fn update_display(&mut self) -> Result<Path, RenderingError>{
        let mut path = Path::new();
        match self.game_state.borrow() {
            AsteroidsGameState::WaitForStart => {
                let wait_text_box = Rect{ origin: Point2D {
                    x: -0.7,
                    y: -0.3,
                    _unit: PhantomData
                }, size: Size2D {
                    width: 1.4,
                    height: 0.2,
                    _unit: PhantomData
                } };
                path = self.text_renderer.print("Press any key to start.", &wait_text_box, ())?;
                while let Some(Event { id, event, time }) = self.gilrs.next_event() {
                    match event {
                        ButtonReleased (_,_) =>{
                            // transitioning to running state
                            // Create shuttle
                            self.player_entity = Some(self.ecs
                                .create_entity()
                                .with(Pose{
                                    x: 0.0,
                                    y: 0.0,
                                    rot: Angle::zero(),
                                })
                                .with(RigidBody {
                                    velocity: vec2(0.0, 0.0),
                                    angular_velocity: Angle::zero(),
                                    last_update: None,
                                    looping: true,
                                })
                                .with(Gun::new())
                                .with(Shuttle::new())
                                .with(Renderable{outline: OutlineFactory::shuttle_outline()})
                                .with(Collider {
                                    collider_type: ColliderType::Player,
                                    collider_outline: OutlineFactory::shuttle_outline(),
                                    collision_handler: player_collision_handler
                                })
                                .with(Score::new())
                                .build());
                            self.active_gamepad = Some(id);
                            self.game_state = AsteroidsGameState::Running;
                            },

                        _ => {}
                    }
                }
            },
            AsteroidsGameState::Running => {
                while let Some(Event { id, event, time }) = self.gilrs.next_event() {
                }
                let mut angle_input =0.0;
                let mut y_axis_input =0.0;
                let mut x_axis_input =0.0;
                let mut firing_input = false;
                match self.active_gamepad {
                    Some(active_gamepad) => {
                        match self.gilrs.gamepad(active_gamepad).axis_data(Axis::RightStickX) {
                            Some(axis_data) => {
                                angle_input = axis_data.value();
                            }
                            _ => {}
                        }

                        match self.gilrs.gamepad(active_gamepad).axis_data(Axis::LeftStickY) {
                            Some(axis_data) => {
                                y_axis_input = axis_data.value();
                            }
                            _ => {}
                        }

                        match self.gilrs.gamepad(active_gamepad).axis_data(Axis::LeftStickX) {
                            Some(axis_data) => {
                                x_axis_input = axis_data.value();
                            }
                            _ => {}
                        }
                        match self.gilrs.gamepad(active_gamepad).button_data(Button::RightTrigger) {
                        Some(button_data) => {
                            firing_input = button_data.is_pressed();
                        }
                        _ => {}
                        }
                    },
                    _ => {println!("No gamepad found.")}
                }
                path = path.merge(render_entities(self.ecs.borrow()).borrow());
                let maybe_telemetry = self.shuttle_update(x_axis_input, y_axis_input, angle_input, firing_input);
                match maybe_telemetry {
                    Some(telemetry) => {
                        self.final_score = telemetry.score;
                        match self.render_ui(telemetry){
                            Ok(ui_path) => {
                                path = path.merge(ui_path.borrow());
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }

                // This dispatches all the systems in parallel (but blocking).
                self.dispatcher.dispatch(&mut self.ecs);
                self.spawn_asteroids();
                self.ecs.maintain();

            },
            AsteroidsGameState::GameEnded => {
                    let game_over_text_box = Rect{ origin: Point2D {
                        x: -0.7,
                        y: -0.3,
                        _unit: PhantomData
                    }, size: Size2D {
                        width: 1.4,
                        height: 0.2,
                        _unit: PhantomData
                    } };
                path = self.text_renderer.print(("Game over\nFinal score: ".to_owned() + &self.final_score.to_string()).as_ref(), &game_over_text_box, ())?;
            },
        }
        Result::Ok(path)
    }
}
