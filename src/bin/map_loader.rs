use game_test;
use std::fs;

fn main() {
    let test_map = fs::read_to_string("assets/maps/cell_blocks.map").unwrap();
    let result = ron::from_str(test_map.as_str());
    let map: game_test::battle_map::CombatMap = result.unwrap();
    println!("RON: {}", ron::to_string(&map).unwrap());
    println!(
        "Pretty RON: {}",
        ron::ser::to_string_pretty(&map, ron::ser::PrettyConfig::default()).unwrap(),
    );
}
