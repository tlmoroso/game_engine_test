use specs::{Component, VecStorage};
use crate::components::BasicTestComponent;
use serde::Deserialize;

pub const BASIC_BOOLEAN_TEST_COMPONENT_LOAD_ID: &str = "basic_boolean_test_component";

#[derive(Deserialize, Debug)]
pub struct BasicBooleanTest {
    pub boolean: bool
}

impl Component for BasicBooleanTest {
    type Storage = VecStorage<Self>;
}

impl BasicTestComponent for BasicBooleanTest {
    const LOAD_ID: &'static str = BASIC_BOOLEAN_TEST_COMPONENT_LOAD_ID;
}