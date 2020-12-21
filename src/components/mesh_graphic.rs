use specs::{Component, World};
use specs::storage::DenseVecStorage;

use coffee::graphics::{Mesh, Window, Shape, Rectangle, Color};
use coffee::load::Task;

use serde::Deserialize;
use crate::components::ComponentLoader;

pub const MESH_GRAPHIC_FILE_ID: &str = "mesh_graphic";

#[derive(Deserialize, Debug)]
struct MeshGraphicLoader {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub stroke_width: f32,
}

#[derive(Debug)]
pub struct MeshGraphic {
    pub mesh: Mesh
}

impl Component for MeshGraphic {
    type Storage = DenseVecStorage<Self>;
}

impl ComponentLoader for MeshGraphicLoader {
    type ComponentType = MeshGraphic;

    fn load_component(self, ecs: &mut World, window: &Window) -> Task<Self::ComponentType> {
        Task::new(|| {
            let mut mesh = Mesh::new();
            mesh.stroke(
                Shape::Rectangle(Rectangle {
                    x: self.x,
                    y: self.y,
                    width: self.width,
                    height: self.height
                }),
                Color::new(
                    self.r,
                    self.g,
                    self.b,
                    self.a
                ),
                self.stroke_width
            );

            Ok(MeshGraphic {
                mesh
            })
        })
    }
}