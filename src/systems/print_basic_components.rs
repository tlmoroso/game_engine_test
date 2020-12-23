use specs::{System, ReadStorage, Join, Entities};
use crate::components::basic_boolean_test::BasicBooleanTest;
use crate::components::basic_number_test::BasicNumberTest;
use crate::components::basic_text_test::BasicTextTest;
use crate::components::basic_vec_test::BasicVectorTest;
use crate::components::basic_map_test::BasicMapTest;

pub struct PrintBasicComponents;

impl<'a> System<'a> for PrintBasicComponents {
    type SystemData = (
        ReadStorage<'a, BasicBooleanTest>,
        ReadStorage<'a, BasicNumberTest>,
        ReadStorage<'a, BasicTextTest>,
        ReadStorage<'a, BasicVectorTest>,
        ReadStorage<'a, BasicMapTest>,
        Entities<'a>
    );

    fn run(&mut self, (bools, numbers, texts, vecs, maps, entities): Self::SystemData) {
        for (bool, number, text, vec, map, entity) in (&bools, &numbers, &texts, &vecs, &maps, &entities).join() {
            println!("Entity:\n\t{:?}", entity);
            println!("Components:");
            println!("\tBasicBooleanTest:\n\t\t{:?}", bool);
            println!("\tBasicNumberTest:\n\t\t{:?}", number);
            println!("\tBasicTextTest:\n\t\t{:?}", text);
            println!("\tBasicVectorTest:\n\t\t{:?}", vec);
            println!("\tBasicMapTest:\n\t\t{:?}", map);
        }
    }
}