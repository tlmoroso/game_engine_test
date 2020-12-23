pub mod basic_number_test;
pub mod basic_boolean_test;
pub mod basic_vec_test;
pub mod basic_text_test;
pub mod basic_map_test;

use game_engine::components::{ComponentMux, ComponentLoader};
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
use crate::globals::TestGlobalError::LoadIDMatchError;
use crate::components::basic_number_test::{BasicNumberTest, BASIC_NUMBER_TEST_COMPONENT_LOAD_ID};
use crate::components::basic_boolean_test::{BasicBooleanTest, BASIC_BOOLEAN_TEST_COMPONENT_LOAD_ID};
use crate::components::basic_vec_test::{BasicVectorTest, BASIC_VECTOR_TEST_COMPONENT_LOAD_ID};
use crate::components::basic_text_test::{BasicTextTest, BASIC_TEXT_TEST_COMPONENT_LOAD_ID};
use crate::components::basic_map_test::{BasicMapTest, BASIC_MAP_TEST_COMPONENT_LOAD_ID};
use crate::components::ComponentError::ComponentPoisonError;
use specs::world::LazyBuilder;

pub struct BasicTestComponentMux {}

impl ComponentMux for BasicTestComponentMux {
    fn map_json_to_loader(json: JSONLoad) -> Result<Box<dyn ComponentLoader>> {
        return match json.load_type_id.as_str() {
            BASIC_NUMBER_TEST_COMPONENT_LOAD_ID => Ok(Box::new(BasicTestComponentLoader::<BasicNumberTest>::new(json)?)),
            BASIC_BOOLEAN_TEST_COMPONENT_LOAD_ID => Ok(Box::new(BasicTestComponentLoader::<BasicBooleanTest>::new(json)?)),
            BASIC_TEXT_TEST_COMPONENT_LOAD_ID => Ok(Box::new(BasicTestComponentLoader::<BasicTextTest>::new(json)?)),
            BASIC_VECTOR_TEST_COMPONENT_LOAD_ID => Ok(Box::new(BasicTestComponentLoader::<BasicVectorTest>::new(json)?)),
            BASIC_MAP_TEST_COMPONENT_LOAD_ID => Ok(Box::new(BasicTestComponentLoader::<BasicMapTest>::new(json)?)),
            _ => Err(anyhow::Error::new(
                LoadIDMatchError {
                    load_type_id: json.load_type_id
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

impl<T: BasicTestComponent + for<'de> Deserialize<'de>> ComponentLoader for BasicTestComponentLoader<T>
    where <T as specs::Component>::Storage: std::default::Default {
    fn load_component<'b>(&self, builder: LazyBuilder<'b>, ecs: &World, window: &Window) -> Result<LazyBuilder<'b>> {
        // println!("Entered load_component for {:?}", self.component_name);
        // let mut ecs = ecs.write()
        //     .map_err( |e| {
        //         anyhow::Error::new(
        //             ComponentPoisonError {
        //                 var_name: stringify!(ecs).to_string(),
        //                 source_string: e.to_string()
        //             }
        //         )
        //     })?;
        // println!("acquired lock for ecs");

        // ecs.register::<T>();
        // println!("registered component with ecs");

        Ok(
            builder.with(
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

#[derive(Error, Debug)]
pub enum ComponentError {
    #[error("Failed to get lock of variable: {var_name:?}.")]
    ComponentPoisonError {
        var_name: String,
        source_string: String
    }
}