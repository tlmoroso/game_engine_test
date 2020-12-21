use std::sync::{Arc, RwLock};
use crate::input::TestCustomInput;
use crate::globals::{JSON_FILE, LOAD_PATH, JSON_ASSETS_DIR};
use crate::scenes::{scene_factory, SCENE_STACKS_DIR};
use game_engine::game::GameWrapper;
use game_engine::scenes::scene_stack::{SceneStack, SceneStackLoader};
use game_engine::scenes::SCENES_DIR;
use crate::scenes::basic_test_scene::BASIC_TEST_SCENE_FILE_ID;
use coffee::graphics::Window;
use coffee::load::Task;
use specs::{World, WorldExt};

pub struct BasicTestGameWrapper {}

impl GameWrapper<TestCustomInput> for BasicTestGameWrapper {
    fn load(window: &Window) -> Task<(Arc<RwLock<World>>, SceneStack<TestCustomInput>)> {
        let mut ecs = Arc::new(RwLock::new(World::new()));

        let loader = SceneStackLoader::new(
            [
                LOAD_PATH,
                JSON_ASSETS_DIR,
                SCENE_STACKS_DIR,
                "basic_test_scene_stack",
                JSON_FILE
            ].join(""),
            scene_factory
        );

        loader.load(ecs.clone(), window)
            .map(|scene_stack| {
                return (ecs, scene_stack)
            })
    }
}