use crate::input::TestCustomInput;
use anyhow::{Result, Error};
use game_engine::load::JSONLoad;
use game_engine::scenes::SceneLoader;
use crate::scenes::basic_test_scene::{BasicTestSceneLoader, BASIC_TEST_SCENE_FILE_ID};
use crate::globals::TestGlobalError::LoadIDMatchError;

pub mod basic_test_scene;

pub const SCENES_DIR: &str = "scenes/";
pub const SCENE_STACKS_DIR: &str = "scene_stacks/";

pub fn scene_factory(json: JSONLoad) -> Result<Box<dyn SceneLoader<TestCustomInput>>> {
    return match json.load_type_id.as_str() {
        BASIC_TEST_SCENE_FILE_ID => {
            Ok(Box::new(
                BasicTestSceneLoader::new(json)?
            ))
        }
        _ => Err(Error::new(LoadIDMatchError {
            expected_id: format!("One of: {:?}", vec!(BASIC_TEST_SCENE_FILE_ID)),
            actual_id: json.load_type_id
        }))
    }
}