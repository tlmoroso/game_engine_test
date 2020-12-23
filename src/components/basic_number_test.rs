use specs::{Component, VecStorage};
use crate::components::BasicTestComponent;
use serde::Deserialize;

pub const BASIC_NUMBER_TEST_COMPONENT_LOAD_ID: &str = "basic_number_test_component";

#[derive(Deserialize, Debug)]
pub struct BasicNumberTest {
    pub number: u32
}

impl Component for BasicNumberTest {
    type Storage = VecStorage<Self>;
}

impl BasicTestComponent for BasicNumberTest {
    const LOAD_ID: &'static str = BASIC_NUMBER_TEST_COMPONENT_LOAD_ID;
}