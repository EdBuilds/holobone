use lyon::algorithms::path::iterator::*;
use lyon::algorithms::path::math::Point;
use lyon::algorithms::path::PathSlice;
use lyon::algorithms::walk::{walk_along_path, RegularPattern};
use pruif::Sample;
use std::f64;

#[derive(Debug)]
pub struct LaserPoint {
    pub on: bool,
    pub x: f64,
    pub y: f64,
}

pub struct Tracer {
    // TODO: add velo, disp, acc limits back when it's used again
}
impl Tracer {
    pub fn new(_max_displacement: f64, _max_velocity: f64, _max_acceleration: f64) -> Self {
        Self {}
    }

    pub fn trace_path(&self, path: PathSlice) -> Vec<Sample> {
        let mut dots = Vec::new();
        let mut pattern = RegularPattern {
            callback: &mut |position: Point, _tangent, _distance| {
                dots.push(convert_to_sample(LaserPoint {
                    on: true,
                    x: position.x as f64,
                    y: position.y as f64,
                }));
                true // Return true to continue walking the path.
            },
            // Invoke the callback above at a regular interval of 3 units.
            interval: 0.01,
        };

        let tolerance = 0.01; // The path flattening tolerance.
        let start_offset = 0.0; // Start walking at the beginning of the path.
        walk_along_path(path.iter().flattened(tolerance), start_offset, &mut pattern);
        dots
    }
}

fn convert_to_sample(point: LaserPoint) -> Sample {
    Sample {
        voltage_x: ((point.x) * -5f64) as f32,
        voltage_y: ((point.y) * -5f64) as f32,
        laser_on: point.on,
    }
}
