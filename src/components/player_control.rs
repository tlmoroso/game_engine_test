use specs::{Component, World};
use specs::storage::HashMapStorage;

use coffee::graphics::Window;
use coffee::load::Task;

use serde::Deserialize;
use crate::components::ComponentLoader;

pub const PLAYER_CONTROL_FILE_ID: &str = "player_control";

#[derive(Deserialize, Debug)]
pub struct PlayerControl {}

impl Component for PlayerControl {
    type Storage = HashMapStorage<Self>;
}

impl ComponentLoader for PlayerControl {
    type ComponentType = Self;

    fn load_component(self, ecs: &mut World, window: &Window) -> Task<Self::ComponentType> {
        Task::new(|| {
            Ok(PlayerControl{})
        })
    }
}