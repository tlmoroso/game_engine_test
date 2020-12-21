use coffee::graphics::{Window, Frame, Text};
use coffee::{Timer};
use coffee::load::{Task, Join};

use serde::Deserialize;
use serde_json::{Value, from_value};

use std::io::ErrorKind;

use anyhow::{Result, Error};

use std::sync::{Arc, RwLock};

use game_engine::load::{JSONLoad, load_entity_vec, build_task_error};
use game_engine::scenes::{Scene, SceneLoaderJSON, SceneLoader};
use game_engine::scenes::scene_stack::SceneTransition;
use crate::input::TestCustomInput;
use specs::{World, Entity};
use crate::components::BasicTestComponentMux;
use crate::globals::TestGlobalError::{LoadIDMatchError, ConvertJSONError};

#[derive(Deserialize, Debug)]
pub struct BasicTestSceneLoader {
    scene_json: JSONLoad,
}

pub const BASIC_TEST_SCENE_FILE_ID: &str = "basic_test_scene";

impl BasicTestSceneLoader {
    pub fn new(json: JSONLoad) -> Result<Self> {
        return if json.load_type_id == BASIC_TEST_SCENE_FILE_ID {
            Ok(Self {
                scene_json: json
            })
        } else {
            Err(Error::new(LoadIDMatchError {
                load_type_id: json.load_type_id
            }))
        }
    }
}

impl SceneLoader<TestCustomInput> for BasicTestSceneLoader {
    fn load_scene(&self, ecs: Arc<RwLock<World>>, window: &Window) -> Task<Box<dyn Scene<TestCustomInput>>> {
        let scene_json: SceneLoaderJSON = map_err_return!(
            from_value(self.scene_json.actual_value.clone()),
            |e| {
                build_task_error(
                    ConvertJSONError {
                        value: self.scene_json.actual_value.clone(),
                        into_type: stringify!(SceneLoaderJSON).to_string(),
                        source: e
                    },
                    ErrorKind::InvalidData
                )
            }
        );

        let scene_values: BasicTestSceneJSON = map_err_return!(
            from_value(scene_json.scene_values.clone()),
            |e| {
                panic!("Error trying to convert json value into scene values: {}", scene_json.scene_values);
            });

        load_entity_vec::<BasicTestComponentMux>(&scene_json.entity_paths, ecs, window)
            .map(move |entity_vec| {
                Box::new(
                    BasicTestScene {
                        entities: entity_vec,
                        text: scene_values.text,
                        frame: scene_values.frame
                    }
                ) as Box<dyn Scene<TestCustomInput>>
            })
    }
}

#[derive(Debug)]
pub struct BasicTestScene {
    entities: Vec<Entity>,
    text: String,
    frame: usize
}

#[derive(Deserialize, Debug)]
struct BasicTestSceneJSON {
    text: String,
    frame: usize
}

impl Scene<TestCustomInput> for BasicTestScene {
    fn update(&mut self, ecs: Arc<RwLock<World>>) -> Result<SceneTransition<TestCustomInput>> {
        self.frame = self.frame + 1;

        Ok(SceneTransition::NONE)
    }

    fn draw(&mut self, ecs: Arc<RwLock<World>>, frame: &mut Frame, timer: &Timer) -> Result<()> {
        if self.frame % 60 == 0 {
            println!("NUM FRAMES: {}", self.frame);
        }
        return Ok(())
    }

    fn interact(&mut self, ecs: Arc<RwLock<World>>, input: &mut TestCustomInput, window: &mut Window) -> Result<()> {
        return Ok(())
    }

    fn get_name(&self) -> String {
        "BASIC TEST SCENE".to_string()
    }

    fn is_finished(&self) -> Result<bool> {
        return if self.frame == 300 {
            println!("{}", self.get_name());
            println!("Entities: {:?}", self.entities);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}