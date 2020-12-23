use specs::{Component, VecStorage};
use crate::components::BasicTestComponent;
use serde::Deserialize;

pub const BASIC_VECTOR_TEST_COMPONENT_LOAD_ID: &str = "basic_vector_test_component";

#[derive(Deserialize, Debug)]
pub struct BasicVectorTest {
    pub vector: Vec<u32>
}

impl Component for BasicVectorTest {
    type Storage = VecStorage<Self>;
}

impl BasicTestComponent for BasicVectorTest {
    const LOAD_ID: &'static str = BASIC_VECTOR_TEST_COMPONENT_LOAD_ID;
}