use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum Command {
    AsteroidsGame,
    RenderSVG,
}
