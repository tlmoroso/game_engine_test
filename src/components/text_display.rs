use specs::{Component, World, Builder};
use specs::storage::DenseVecStorage;

use coffee::graphics::{Point, Color, HorizontalAlignment, VerticalAlignment, Window};
// use coffee::load::Task;
use anyhow::Result;
use serde::Deserialize;
use crate::components::ComponentLoader;
use game_engine::load::JSONLoad;
use crate::globals::TestGlobalError::{LoadIDMatchError, ConvertJSONError};
use serde_json::from_value;
use specs::world::LazyBuilder;

pub const TEXT_DISPLAY_FILE_ID: &str = "text_display";

const H_ALIGN: HorizontalAlignment = HorizontalAlignment::Center;
const V_ALIGN: VerticalAlignment = VerticalAlignment::Center;

#[derive(Deserialize, Debug, Clone)]
struct TextDisplayJSON {
    pub content: Vec<String>,
    pub position_x: f32,
    pub position_y: f32,
    pub bounds_x: f32,
    pub bounds_y: f32,
    pub size: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub font: String,
}

#[derive(Debug)]
pub struct TextDisplay {
    pub content: Vec<String>,
    pub position: Point,
    pub bounds: (f32, f32),
    pub size: f32,
    pub color: Color,
    pub h_align: HorizontalAlignment,
    pub v_align: VerticalAlignment,
    pub font: String,
}

impl From<TextDisplayJSON> for TextDisplay {
    fn from(json: TextDisplayJSON) -> Self {
        TextDisplay {
            content: json.content,
            position: Point::from([json.position_x, json.position_y]),
            bounds: (json.bounds_x, json.bounds_y),
            size: json.size,
            color: Color::new(
                json.r,
                json.g,
                json.b,
                json.a
            ),
            h_align: H_ALIGN,
            v_align: V_ALIGN,
            font: json.font
        }
    }
}

impl Component for TextDisplay {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug)]
pub struct TextDisplayLoader {
    text_display_json: TextDisplayJSON
}

impl ComponentLoader for TextDisplayLoader {
    fn from_json(json: JSONLoad) -> Result<Self> where Self: Sized {
        if json.load_type_id == TEXT_DISPLAY_FILE_ID {
            let text_display_json = from_value(json.actual_value.clone())
                .map_err(|e| {
                    anyhow::Error::new(
                        ConvertJSONError {
                            value: json.actual_value,
                            into_type: stringify!(TextdisplayJSON).to_string(),
                            source: e
                        }
                    )
                })?;

            Ok(TextDisplayLoader {
                text_display_json
            })
        } else {
            Err( anyhow::Error::new(
                LoadIDMatchError {
                    expected_id: TEXT_DISPLAY_FILE_ID.to_string(),
                    actual_id: json.load_type_id
                }
            ))
        }
    }

    fn load_component<'a>(&self, builder: LazyBuilder<'a>, ecs: &World, _window: &Window) -> Result<LazyBuilder<'a>> {
        Ok(builder.with(TextDisplay::from(self.text_display_json.clone())))
    }

    fn set_value(&mut self, new_value: JSONLoad) -> Result<()> {
        if new_value.load_type_id == TEXT_DISPLAY_FILE_ID {
            self.text_display_json = from_value(new_value.actual_value.clone())
                .map_err(|e| {
                    ConvertJSONError {
                        value: new_value.actual_value,
                        into_type: stringify!(TextDisplayJSON).to_string(),
                        source: e
                    }
                })?;

            Ok(())
        } else {
            Err(
                anyhow::Error::new(
                    LoadIDMatchError {
                        expected_id: TEXT_DISPLAY_FILE_ID.to_string(),
                        actual_id: new_value.load_type_id
                    }
                )
            )
        }
    }

    fn get_component_name(&self) -> String {
        return TEXT_DISPLAY_FILE_ID.to_string()
    }
}