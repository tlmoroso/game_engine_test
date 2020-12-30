use specs::{System, ReadStorage, Join, Write};
use crate::components::mesh_graphic::MeshGraphic;
use crate::components::text_display::TextDisplay;
use coffee::graphics::{Frame, Text};
use game_engine::globals::FontDict;
use std::ops::Deref;
use crate::globals::NORMAL_FONT;
use std::borrow::BorrowMut;

pub struct DrawTextBox<'a, 'b> {
    pub frame: &'a mut Frame<'b>,
}

impl<'a, 'b> System<'a> for DrawTextBox<'a, 'b> {
    type SystemData = (
        Write<'a, FontDict>,
        ReadStorage<'a, MeshGraphic>,
        ReadStorage<'a, TextDisplay>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut font_dict, mesh_graphics, text_displays) = data;

        for (mesh_graphic, text_display) in (&mesh_graphics, &text_displays).join() {
            mesh_graphic.mesh.draw(self.frame.as_target().borrow_mut());
            let text = Text {
                content: text_display.content.get(0).expect(format!("Failed to get content from text_display: {:?} at index: {}", text_display, 0).as_str()),
                position: text_display.position.clone(),
                bounds: text_display.bounds,
                size: text_display.size,
                color: text_display.color,
                horizontal_alignment: text_display.h_align,
                vertical_alignment: text_display.v_align
            };

            let mut font = font_dict.0.get_mut(text_display.font.as_str())
                .expect(format!("Failed to get font: {:?} from FontDict in DrawTextBox system", text_display.font).as_str());

            font.add(text);
            font.draw(self.frame.as_target().borrow_mut());
        }
    }
}