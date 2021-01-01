use lyon::geom::euclid::Angle;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Clone)]
pub struct Pose {
    pub x: f32,
    pub y: f32,
    pub rot: Angle<f32>,
}
