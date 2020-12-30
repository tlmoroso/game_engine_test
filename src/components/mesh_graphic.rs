use specs::{Component, World, Builder};
use specs::storage::DenseVecStorage;

use coffee::graphics::{Mesh, Window, Shape, Rectangle, Color};
use coffee::load::Task;

use serde::Deserialize;
use crate::components::ComponentLoader;
use game_engine::load::JSONLoad;
use specs::world::LazyBuilder;

use anyhow::Result;
use serde_json::from_value;
use crate::globals::TestGlobalError::{ConvertJSONError, LoadIDMatchError};
use game_engine::load::LoadError::LoadIDError;

pub const MESH_GRAPHIC_LOAD_ID: &str = "mesh_graphic";

#[derive(Deserialize, Debug, Copy, Clone)]
struct MeshGraphicJSON {
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

impl From<MeshGraphicJSON> for MeshGraphic {
    fn from(mesh_json: MeshGraphicJSON) -> Self {
        let mut mesh = Mesh::new();
        mesh.stroke(
            Shape::Rectangle(Rectangle {
                x: mesh_json.x,
                y: mesh_json.y,
                width: mesh_json.width,
                height: mesh_json.height
            }),
            Color::new(
                mesh_json.r,
                mesh_json.g,
                mesh_json.b,
                mesh_json.a
            ),
            mesh_json.stroke_width
        );

        MeshGraphic {
            mesh
        }
    }
}

#[derive(Debug)]
pub struct MeshGraphicLoader {
    mesh_json: MeshGraphicJSON
}

impl ComponentLoader for MeshGraphicLoader {
    fn from_json(json: JSONLoad) -> Result<Self> where Self: Sized {
        return if json.load_type_id == MESH_GRAPHIC_LOAD_ID {
            let mesh_json: MeshGraphicJSON = from_value(json.actual_value.clone())
                .map_err(|e| {
                    anyhow::Error::new(
                        ConvertJSONError {
                            value: json.actual_value,
                            into_type: "MeshGraphicJSON".to_string(),
                            source: e
                        }
                    )
                })?;
            
            Ok(MeshGraphicLoader{mesh_json})
        } else {
            Err(
                anyhow::Error::new(
                    LoadIDMatchError {
                        expected_id: MESH_GRAPHIC_LOAD_ID.to_string(),
                        actual_id: json.load_type_id
                    }
                )
            )
        }
    }

    fn load_component<'a>(&self, builder: LazyBuilder<'a>, ecs: &World, window: &Window) -> Result<LazyBuilder<'a>> {
        let mesh_graphic: MeshGraphic = self.mesh_json.into();

        Ok(builder.with(mesh_graphic))
    }

    fn set_value(&mut self, new_value: JSONLoad) -> Result<()> {
        if new_value.load_type_id == MESH_GRAPHIC_LOAD_ID {
            self.mesh_json = from_value(new_value.actual_value.clone())
                .map_err(|e| {
                    anyhow::Error::new(
                        ConvertJSONError {
                            value: new_value.actual_value,
                            into_type: stringify!(MeshGraphicJSON).to_string(),
                            source: e
                        }
                    )
                })?;

            Ok(())
        } else {
            Err(
                anyhow::Error::new(LoadIDMatchError {
                    expected_id: MESH_GRAPHIC_LOAD_ID.to_string(),
                    actual_id: new_value.load_type_id
                })
            )
        }
    }

    fn get_component_name(&self) -> String {
        return "Mesh Graphic".to_string()
    }
}