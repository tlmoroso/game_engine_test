use std::sync::{Arc, RwLock};
use crate::input::TestCustomInput;
use crate::globals::{JSON_FILE, LOAD_PATH, JSON_ASSETS_DIR, FONTS_DIR};
use crate::scenes::{scene_factory, SCENE_STACKS_DIR};
use game_engine::game::GameWrapper;
use game_engine::scenes::scene_stack::{SceneStack, SceneStackLoader};
use game_engine::scenes::SCENES_DIR;
use crate::scenes::basic_test_scene::BASIC_TEST_SCENE_FILE_ID;
use coffee::graphics::Window;
use coffee::load::{Task, Join};
use specs::{World, WorldExt};
use crate::components::basic_boolean_test::BasicBooleanTest;
use crate::components::basic_number_test::BasicNumberTest;
use crate::components::basic_text_test::BasicTextTest;
use crate::components::basic_vec_test::BasicVectorTest;
use crate::components::basic_map_test::BasicMapTest;
use crate::components::mesh_graphic::MeshGraphic;
use crate::components::text_display::TextDisplay;
use crate::globals::TestGlobalError::ECSWriteError;
use game_engine::components::drawables::Drawable;
use kira::manager::AudioManagerSettings;
use game_engine::components::audibles::default_sound::DefaultSound;
use game_engine::globals::font_dict::{FontDictLoader, FONT_DICT_LOAD_ID};
use game_engine::globals::image_dict::{ImageDictLoader, IMAGE_DICT_LOAD_ID};
use game_engine::globals::audio_controller::{AudioControllerLoader, AUDIO_CONTROLLER_LOAD_ID};

pub struct BasicTestGameWrapper {}

impl GameWrapper<TestCustomInput> for BasicTestGameWrapper {
    fn register_components(ecs: &mut World) {
        ecs.register::<BasicBooleanTest>();
        ecs.register::<BasicNumberTest>();
        ecs.register::<BasicTextTest>();
        ecs.register::<BasicVectorTest>();
        ecs.register::<BasicMapTest>();
        ecs.register::<Drawable>();
        ecs.register::<DefaultSound>();
    }

    fn load(window: &Window) -> Task<(Arc<RwLock<World>>, SceneStack<TestCustomInput>)> {
        let ecs = Arc::new(RwLock::new(World::new()));

        let font_dict_loader = FontDictLoader::new(
            [
                LOAD_PATH,
                JSON_ASSETS_DIR,
                FONT_DICT_LOAD_ID,
                JSON_FILE
            ].join("")
        );

        let image_dict_loader = ImageDictLoader::new(
            [
                LOAD_PATH,
                JSON_ASSETS_DIR,
                IMAGE_DICT_LOAD_ID,
                JSON_FILE
            ].join("")
        );

        let audio_controller_loader = AudioControllerLoader::new(
            [
                LOAD_PATH,
                JSON_ASSETS_DIR,
                AUDIO_CONTROLLER_LOAD_ID,
                JSON_FILE
            ].join("")
        );

        let loader = SceneStackLoader::new(
            [
                LOAD_PATH,
                JSON_ASSETS_DIR,
                SCENE_STACKS_DIR,
                "basic_test_scene_stack",
                JSON_FILE
            ].join(""),
            scene_factory
        );

        (
            font_dict_loader.load(),
            image_dict_loader.load(),
            audio_controller_loader.load(AudioManagerSettings::default()),
            loader.load(ecs.clone(), window)
        ).join()
            .map(|(font_dict, image_dict, audio_controller, scene_stack)| {
                let mut mut_ecs = ecs.write()
                    .map_err(|e| {
                        ECSWriteError {
                            source_string: e.to_string()
                        }
                    }).unwrap();

                mut_ecs.insert(font_dict);
                mut_ecs.insert(image_dict);
                mut_ecs.insert(audio_controller);
                std::mem::drop(mut_ecs); // Manually drop RwLock so we can move ecs in return

                return (ecs, scene_stack)
            })
    }
}