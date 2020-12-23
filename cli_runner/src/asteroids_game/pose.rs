use lyon::geom::euclid::Angle;
use specs_derive::Component;
use specs::prelude::*;

#[derive(Component, Clone)]
pub struct Pose {
    pub x: f32,
    pub y: f32,
    pub rot: Angle<f32>,
}
