use specs::{Component, DenseVecStorage, World};

use coffee::graphics::{Image, Sprite, Point, Rectangle, Window};
use coffee::load::Task;

use crate::components::position::Position;
use crate::globals::ImageDict;

use serde::{Deserialize};
use crate::components::ComponentLoader;

pub const ANIMATION_FILE_ID: &str = "animation";

#[derive(Deserialize, Debug, Clone)]
struct AnimationLoader {
    pub image: String,
    pub current_frame: u16,
    pub start_frame: u16,
    pub end_frame: u16,
    pub total_frames: u16,
    pub dimensions_x: u16,
    pub dimensions_y: u16,
    pub scale_x: f32,
    pub scale_y: f32
}

#[derive(Debug, Clone)]
pub struct Animation {
    pub image: Image,
    pub current_frame: u16, // frames are 1-indexed
    pub start_frame: u16,
    pub end_frame: u16,
    pub total_frames: u16,
    pub dimensions: (u16, u16),
    pub scale: (f32, f32),
}

impl Component for Animation {
    type Storage = DenseVecStorage<Self>;
}

impl ComponentLoader for AnimationLoader {
    type ComponentType = Animation;

    fn load_component(self, ecs: &mut World, window: &Window) -> Task<Self::ComponentType> {
        let image = ecs.get_mut::<ImageDict>().unwrap()
            .0
            .get(self.image.as_str());

        if let Some(handle) = image {
            Task::new(|| { Ok(
                Animation {
                    image: handle.clone(),
                    current_frame: self.current_frame,
                    start_frame: self.start_frame,
                    end_frame: self.end_frame,
                    total_frames: self.total_frames,
                    dimensions: (self.dimensions_x, self.dimensions_y),
                    scale: (self.scale_x, self.scale_y)
                })
            })
        } else {
            Image::load(self.image)
                .map(|new_image| {
                    Animation {
                        image: new_image,
                        current_frame: self.current_frame,
                        start_frame: self.start_frame,
                        end_frame: self.end_frame,
                        total_frames: self.total_frames,
                        dimensions: (self.dimensions_x, self.dimensions_y),
                        scale: (self.scale_x, self.scale_y)
                    }
                })
        }
    }
}

impl Animation {
    pub fn create_sprite(&mut self, pos: &Position) -> Sprite {
        let frame_width = self.dimensions.0/self.total_frames;
        let frame_height = self.dimensions.1;
        let frame_x = frame_width * (self.current_frame - 1);
        let frame_y = pos.y;


        self.current_frame += 1;

        Sprite {
            source: Rectangle {
                x: frame_x,
                y: frame_y,
                width: frame_width,
                height: frame_height,
            },
            position: Point::new(pos.x.into(), pos.y.into()),
            scale: self.scale,
        }
    }
}
