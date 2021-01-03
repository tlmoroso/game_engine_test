#[macro_use]
extern crate game_engine;

use crate::game::BasicTestGameWrapper;
use crate::input::TestCustomInput;

use coffee::graphics::WindowSettings;
use coffee::Game;

use game_engine::game::MyGame;

mod scenes;
mod game;
mod globals;
mod input;
mod components;
mod systems;

fn main() -> coffee::Result<()> {
    MyGame::<BasicTestGameWrapper, TestCustomInput, ()>::run( WindowSettings {
        title: String::from("HELLO WORLD"),
        size: (1280, 1024),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
}
