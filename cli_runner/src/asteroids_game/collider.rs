use specs_derive::Component;
use specs::prelude::*;
use lyon_path::Path;

#[derive(Clone, Copy)]
pub enum ColliderType{
    Projectile,
    Enemy,
    Player
}

pub enum CollisionHandlerError{
    MissingComponent,
    //NotImplemented,
}

//TODO: return the entities that needs to be deleted!
pub type CollisionHandler = fn(current_entity: Entity, other_entity: Entity, world:&mut World) -> Result<(), CollisionHandlerError>;

#[derive(Component)]
pub struct Collider {
    pub collider_type: ColliderType,
    pub collider_outline: Path,
    pub collision_handler: CollisionHandler
}

