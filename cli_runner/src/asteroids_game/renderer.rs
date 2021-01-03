use lyon_path::Path;
use lyon::math::{point, Point, Vector, Angle};
use specs_derive::Component;
use specs::prelude::*;
use crate::asteroids_game::pose::Pose;
use std::marker::PhantomData;
use lyon::geom::euclid::{Translation2D, Rotation2D};
use std::borrow::Borrow;
pub struct OutlineFactory;
impl OutlineFactory {
    pub fn shuttle_outline() -> Path{
        let mut builder = Path::builder();
        let scale = 0.02;
        builder.move_to(point(0.0, ( 2.0 * scale) as f32));
        builder.line_to(point((scale / 2.0) as f32, 0.0));
        builder.line_to(point(-(scale / 2.0) as f32, 0.0));
        builder.close();
        builder.build()
    }
    pub fn shuttle_shield() -> Path{
        let mut builder = Path::builder();
        let scale = 0.05;
        builder.move_to(point(scale, 0.02));
        builder.arc(Point::new(0.00,0.02), Vector::new(scale, scale), Angle::radians(2.0*std::f32::consts::PI), Angle::radians(0.0));
        builder.close();
        builder.build()
    }
    pub fn asteroid_outline(level: u32) -> Path{
        let mut builder = Path::builder();
        let size = 0.02 * (level as f32);
        builder.move_to(point(-size, -size));
        builder.line_to(point(-size, size));
        builder.line_to(point(size, size));
        builder.line_to(point(size, -size));
        builder.close();
        builder.build()
    }
    pub fn projectile_outline() -> Path{
        let mut builder = Path::builder();
        builder.move_to(point(0.0, 0.0));
        builder.line_to(point(0.0, 0.02));
        builder.close();
        builder.build()
    }

    pub fn projectile_collider_outline() -> Path{
        let mut builder = Path::builder();
        builder.move_to(point(0.005, 0.0));
        builder.line_to(point(0.005, 0.02));
        builder.line_to(point(-0.005, 0.02));
        builder.line_to(point(-0.005, 0.0));
        builder.close();
        builder.build()
    }

}
#[derive(Component)]
pub struct Renderable {
    pub outline: Path
}

pub fn render_entities(ecs: &World) -> Path{
    let mut poses = ecs.read_storage::<Pose>();
    let mut renderables = ecs.read_storage::<Renderable>();
    let mut return_path = Path::new();
    for (pose, renderable) in ( &poses, &renderables).join() {
        let mut path = renderable.outline.transformed(Rotation2D{
            angle: - pose.rot.radians,
            _unit: PhantomData
        }.borrow());
        path = path.transformed(Translation2D{
            x: pose.x,
            y: pose.y,
            _unit: PhantomData
        }.borrow());
        return_path = return_path.merge(&path);
    }
    return_path
}
