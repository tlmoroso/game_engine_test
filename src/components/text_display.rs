use specs::{Component, World};
use specs::storage::DenseVecStorage;

use coffee::graphics::{Point, Color, HorizontalAlignment, VerticalAlignment, Window};
use coffee::load::Task;

use serde::Deserialize;
use crate::components::ComponentLoader;

pub const TEXT_DISPLAY_FILE_ID: &str = "text_display";

const H_ALIGN: HorizontalAlignment = HorizontalAlignment::Center;
const V_ALIGN: VerticalAlignment = VerticalAlignment::Center;

#[derive(Deserialize, Debug)]
struct TextDisplayLoader {
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

impl Component for TextDisplay {
    type Storage = DenseVecStorage<Self>;
}

impl ComponentLoader for TextDisplayLoader {
    type ComponentType = TextDisplay;

    fn load_component(self, _ecs: &mut World, _window: &Window) -> Task<Self::ComponentType> {
        Task::new(|| { Ok(
            TextDisplay {
                content: self.content,
                position: Point::from([self.position_x, self.position_y]),
                bounds: (self.bounds_x, self.bounds_y),
                size: self.size,
                color: Color::new(
                    self.r,
                    self.g,
                    self.b,
                    self.a
                ),
                h_align: H_ALIGN,
                v_align: V_ALIGN,
                font: self.font
            }
        )})
    }
}

// impl ComponentLoader<DenseVecStorage<TextDisplay>> for TextDisplayLoader {}