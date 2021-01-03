pub mod basic_number_test;
pub mod basic_boolean_test;
pub mod basic_vec_test;
pub mod basic_text_test;
pub mod basic_map_test;
pub mod mesh_graphic;
pub mod text_display;

use game_engine::components::{ComponentMux, ComponentLoader, drawables::{DRAWABLE_LOAD_ID, Drawable, DrawableLoader}};
use game_engine::load::JSONLoad;

use anyhow::Result;
use thiserror::Error;
use specs::{Component, VecStorage, EntityBuilder, World, Builder, WorldExt};
use std::fmt::Debug;
use serde::Deserialize;
use std::collections::HashMap;
use serde_json::{Value, from_value};
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};
use coffee::graphics::Window;
use crate::globals::TestGlobalError::{LoadIDMatchError, ConvertJSONError};
use crate::components::basic_number_test::{BasicNumberTest, BASIC_NUMBER_TEST_COMPONENT_LOAD_ID};
use crate::components::basic_boolean_test::{BasicBooleanTest, BASIC_BOOLEAN_TEST_COMPONENT_LOAD_ID};
use crate::components::basic_vec_test::{BasicVectorTest, BASIC_VECTOR_TEST_COMPONENT_LOAD_ID};
use crate::components::basic_text_test::{BasicTextTest, BASIC_TEXT_TEST_COMPONENT_LOAD_ID};
use crate::components::basic_map_test::{BasicMapTest, BASIC_MAP_TEST_COMPONENT_LOAD_ID};
use crate::components::mesh_graphic::{MESH_GRAPHIC_LOAD_ID, MeshGraphicLoader, MeshGraphic};
use crate::components::text_display::{TEXT_DISPLAY_FILE_ID, TextDisplay, TextDisplayLoader};
use crate::components::ComponentError::ComponentPoisonError;
use specs::world::LazyBuilder;
use game_engine::components::audibles::default_sound::{DefaultSoundLoader, DEFAULT_SOUND_LOAD_ID};

pub struct BasicTestComponentMux {}

impl ComponentMux for BasicTestComponentMux {
    fn map_json_to_loader(json: JSONLoad) -> Result<Box<dyn ComponentLoader>> {
        return match json.load_type_id.as_str() {
            BASIC_NUMBER_TEST_COMPONENT_LOAD_ID => Ok(Box::new(BasicTestComponentLoader::<BasicNumberTest>::from_json(json)?)),
            BASIC_BOOLEAN_TEST_COMPONENT_LOAD_ID => Ok(Box::new(BasicTestComponentLoader::<BasicBooleanTest>::from_json(json)?)),
            BASIC_TEXT_TEST_COMPONENT_LOAD_ID => Ok(Box::new(BasicTestComponentLoader::<BasicTextTest>::from_json(json)?)),
            BASIC_VECTOR_TEST_COMPONENT_LOAD_ID => Ok(Box::new(BasicTestComponentLoader::<BasicVectorTest>::from_json(json)?)),
            BASIC_MAP_TEST_COMPONENT_LOAD_ID => Ok(Box::new(BasicTestComponentLoader::<BasicMapTest>::from_json(json)?)),
            DRAWABLE_LOAD_ID => Ok(Box::new(DrawableLoader::from_json(json)?)),
            DEFAULT_SOUND_LOAD_ID=> Ok(Box::new(DefaultSoundLoader::from_json(json)?)),
            _ => Err(anyhow::Error::new(
                LoadIDMatchError {
                    expected_id: "Expected one of basic test components load IDs".to_string(),
                    actual_id: "".to_string()
                }
            ))
        }
    }
}

pub trait BasicTestComponent: Component + Debug + Send + Sync {
    const LOAD_ID: &'static str;
}

#[derive(Debug)]
pub struct BasicTestComponentLoader<T: BasicTestComponent> {
    cached_value: Value,
    component_name: String,
    phantom: PhantomData<T>
}

impl<T: BasicTestComponent + for<'de> Deserialize<'de>> ComponentLoader for BasicTestComponentLoader<T>
    where <T as specs::Component>::Storage: std::default::Default {
    fn from_json(json: JSONLoad) -> Result<Self> where Self: Sized {
        return if json.load_type_id == T::LOAD_ID {
            Ok( Self {
                cached_value: json.actual_value,
                component_name: T::LOAD_ID.to_string(),
                phantom: PhantomData
            })
        } else {
            Err(anyhow::Error::new(
                LoadIDMatchError {
                    expected_id: T::LOAD_ID.to_string(),
                    actual_id: json.load_type_id
                })
            )
        }
    }

    fn load_component<'b>(&self, builder: LazyBuilder<'b>, ecs: &World, window: &Window) -> Result<LazyBuilder<'b>> {
        Ok(
            builder.with(from_value::<T>(self.cached_value.clone())
                .map_err(|e| {
                        anyhow::Error::new(
                            ConvertJSONError {
                                value: self.cached_value.clone(),
                                into_type: "BasicTestComponentLoader::T".to_string(),
                                source: e
                            }
                        )
                    }
                )?
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
                    expected_id: T::LOAD_ID.to_string(),
                    actual_id: new_value.load_type_id
                }
            ))
        }
    }

    fn get_component_name(&self) -> String {
        return self.component_name.clone()
    }
}

#[derive(Error, Debug)]
pub enum ComponentError {
    #[error("Failed to get lock of variable: {var_name:?}.")]
    ComponentPoisonError {
        var_name: String,
        source_string: String
    }
}