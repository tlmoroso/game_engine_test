use game_engine::load::LoadError;
use serde_json::Value;
use thiserror::Error;
use std::sync::{PoisonError, RwLockWriteGuard};
use specs::World;

pub const LOAD_PATH: &str = "assets/";
pub const JSON_FILE: &str = ".json";
pub const JSON_ASSETS_DIR: &str = "json/";
pub const FONTS_DIR: &str = "fonts/";

pub const NORMAL_FONT: &str = "astron_boy";

#[derive(Error, Debug)]
pub enum TestGlobalError {
    #[error("load_type_id: {actual_id:?} did not match expected ID: {expected_id:?}")]
    LoadIDMatchError {
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
    },
    #[error("Error getting write lock for ECS")]
    ECSWriteError {
        source_string: String
    },
    #[error("Error getting read lock for ECS")]
    ECSReadError {
        source_string: String
    }
}