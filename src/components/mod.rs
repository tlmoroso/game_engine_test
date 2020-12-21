use game_engine::components::{ComponentMux, ComponentLoader};
use game_engine::load::JSONLoad;

use anyhow::Result;
use specs::{Component, VecStorage, EntityBuilder, World, Builder};
use std::fmt::Debug;
use serde::Deserialize;
use std::collections::HashMap;
use serde_json::{Value, from_value};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};
use coffee::graphics::Window;
use crate::globals::TestGlobalError::LoadIDMatchError;

pub const BASIC_TEST_NUMBER_COMPONENT_LOAD_ID: &str = "basic_test_number_component";
pub const BASIC_TEST_BOOLEAN_COMPONENT_LOAD_ID: &str = "basic_test_boolean_component";
pub const BASIC_TEST_TEXT_COMPONENT_LOAD_ID: &str = "basic_test_text_component";
pub const BASIC_TEST_VECTOR_COMPONENT_LOAD_ID: &str = "basic_test_vector_component";
pub const BASIC_TEST_MAP_COMPONENT_LOAD_ID: &str = "basic_test_map_component";

pub struct BasicTestComponentMux {}

impl ComponentMux for BasicTestComponentMux {
    fn map_json_to_loader(json: JSONLoad) -> Result<Box<dyn ComponentLoader>> {
        return match json.load_type_id.as_str() {
            BASIC_TEST_NUMBER_COMPONENT_LOAD_ID => Ok(Box::new(BasicTestComponentLoader::<BasicNumberTestComponent>::new(json)?)),
            BASIC_TEST_BOOLEAN_COMPONENT_LOAD_ID => Ok(Box::new(BasicTestComponentLoader::<BasicBooleanTestComponent>::new(json)?)),
            BASIC_TEST_TEXT_COMPONENT_LOAD_ID => Ok(Box::new(BasicTestComponentLoader::<BasicTextTestComponent>::new(json)?)),
            BASIC_TEST_VECTOR_COMPONENT_LOAD_ID => Ok(Box::new(BasicTestComponentLoader::<BasicVectorTestComponent>::new(json)?)),
            BASIC_TEST_MAP_COMPONENT_LOAD_ID => Ok(Box::new(BasicTestComponentLoader::<BasicMapTestComponent>::new(json)?)),
            _ => Err(anyhow::Error::new(
                LoadIDMatchError {
                    load_type_id: json.load_type_id
                }
            ))
        }
    }
}

pub trait BasicTestComponent: Component + Debug + Send + Sync + for<'de> Deserialize<'de> {
    const LOAD_ID: &'static str;
}

#[derive(Deserialize, Debug)]
pub struct BasicNumberTestComponent {
    pub number: u32
}

impl Component for BasicNumberTestComponent {
    type Storage = VecStorage<Self>;
}

impl BasicTestComponent for BasicNumberTestComponent {
    const LOAD_ID: &'static str = BASIC_TEST_NUMBER_COMPONENT_LOAD_ID;
}

#[derive(Deserialize, Debug)]
pub struct BasicBooleanTestComponent {
    pub boolean: bool
}

impl Component for BasicBooleanTestComponent {
    type Storage = VecStorage<Self>;
}

impl BasicTestComponent for BasicBooleanTestComponent {
    const LOAD_ID: &'static str = BASIC_TEST_BOOLEAN_COMPONENT_LOAD_ID;
}

#[derive(Deserialize, Debug)]
pub struct BasicTextTestComponent {
    pub text: String
}

impl Component for BasicTextTestComponent {
    type Storage = VecStorage<Self>;
}

impl BasicTestComponent for BasicTextTestComponent {
    const LOAD_ID: &'static str = BASIC_TEST_TEXT_COMPONENT_LOAD_ID;
}

#[derive(Deserialize, Debug)]
pub struct BasicVectorTestComponent {
    pub vector: Vec<u32>
}

impl Component for BasicVectorTestComponent {
    type Storage = VecStorage<Self>;
}

impl BasicTestComponent for BasicVectorTestComponent {
    const LOAD_ID: &'static str = BASIC_TEST_VECTOR_COMPONENT_LOAD_ID;
}

#[derive(Deserialize, Debug)]
pub struct BasicMapTestComponent {
    pub map: HashMap<u32, String>
}

impl Component for BasicMapTestComponent {
    type Storage = VecStorage<Self>;
}

impl BasicTestComponent for BasicMapTestComponent {
    const LOAD_ID: &'static str = BASIC_TEST_MAP_COMPONENT_LOAD_ID;
}

#[derive(Debug)]
pub struct BasicTestComponentLoader<T: BasicTestComponent> {
    cached_value: Value,
    component_name: String,
    phantom: PhantomData<T>
}

impl<T: BasicTestComponent> BasicTestComponentLoader<T> {
    fn new(json: JSONLoad) -> Result<Self> {
        return if json.load_type_id == T::LOAD_ID {
            Ok( Self {
                cached_value: json.actual_value,
                component_name: T::LOAD_ID.to_string(),
                phantom: PhantomData
            })
        } else {
            Err(anyhow::Error::new(
                LoadIDMatchError {
                    load_type_id: json.load_type_id,
                })
            )
        }
    }
}

impl<T: BasicTestComponent> ComponentLoader for BasicTestComponentLoader<T> {
    fn load_component<'b>(&self, entity_task: EntityBuilder<'b>, ecs: Arc<RwLock<World>>, window: &Window) -> Result<EntityBuilder<'b>> {
        Ok(
            entity_task.with(
                from_value::<T>(self.cached_value.clone())
                    .map_err(|e| {
                        anyhow::Error::new(e)
                    })?
            )
        )
    }

    fn set_value(&mut self, new_value: JSONLoad) -> Result<()> {
        return if new_value.load_type_id == T::LOAD_ID {
            self.cached_value = new_value.actual_value;
            Ok(())
        } else {
            Err(anyhow::Error::new(
                LoadIDMatchError {
                    load_type_id: new_value.load_type_id,
                }
            ))
        }
    }

    fn get_component_name(&self) -> String {
        return self.component_name.clone()
    }
}