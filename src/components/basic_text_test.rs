use specs::{Component, VecStorage};
use crate::components::{BasicTestComponent};
use serde::Deserialize;

pub const BASIC_TEXT_TEST_COMPONENT_LOAD_ID: &str = "basic_text_test_component";

#[derive(Deserialize, Debug)]
pub struct BasicTextTest {
    pub text: String
}

impl Component for BasicTextTest {
    type Storage = VecStorage<Self>;
}

impl BasicTestComponent for BasicTextTest {
    const LOAD_ID: &'static str = BASIC_TEXT_TEST_COMPONENT_LOAD_ID;
}