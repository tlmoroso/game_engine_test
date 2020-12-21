use game_engine::load::LoadError;
use serde_json::Value;
use thiserror::Error;

pub const LOAD_PATH: &str = "assets/";
pub const JSON_FILE: &str = ".json";
pub const JSON_ASSETS_DIR: &str = "json/";

#[derive(Error, Debug)]
pub enum TestGlobalError {
    #[error("Failed to match load_type_id: {load_type_id}")]
    LoadIDMatchError {
        load_type_id: String
    },
    #[error("Given JSONLoad's load id: {actual_id} did not match TEST_COMPONENT_LOAD_ID: {expected_id}")]
    TestComponentLoadIDError {
        expected_id: String,
        actual_id: String
    },
    #[error("Failed to load json from: {file_path}")]
    JSONLoadError {
        file_path: String,
        source: LoadError
    },
    #[error("Error converting serde_json::Value: {value} into {into_type}")]
    ConvertJSONError {
        value: Value,
        into_type: String,
        source: serde_json::error::Error
    }
}