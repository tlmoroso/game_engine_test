use specs::{Component, World};
use specs::storage::VecStorage;

use coffee::graphics::Window;
use coffee::load::Task;

use serde::Deserialize;
use crate::components::ComponentLoader;

pub const POSITION_FILE_ID: &str = "position";

#[derive(Deserialize, Debug)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl ComponentLoader for Position {
    type ComponentType = Self;

    fn load_component(self, _ecs: &mut World, _window: &Window) -> Task<Self::ComponentType> {
        Task::new(|| {
            Ok(self)
        })
    }
}

// impl ComponentLoader<VecStorage<Position>> for Position {}