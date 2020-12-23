use std::collections::HashMap;
use specs::{Component, VecStorage};
use crate::components::BasicTestComponent;
use serde::Deserialize;

pub const BASIC_MAP_TEST_COMPONENT_LOAD_ID: &str = "basic_map_test_component";

#[derive(Deserialize, Debug)]
pub struct BasicMapTest {
    pub map: HashMap<u32, String>
}

impl Component for BasicMapTest {
    type Storage = VecStorage<Self>;
}

impl BasicTestComponent for BasicMapTest {
    const LOAD_ID: &'static str = BASIC_MAP_TEST_COMPONENT_LOAD_ID;
}