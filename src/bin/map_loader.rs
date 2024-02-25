use game_test;
use std::fs;

fn main() {
    let test_chars = fs::read_to_string("assets/characters.characters").unwrap();
    let players: game_test::characters::SaveCharacters = ron::from_str(test_chars.as_str()).unwrap();
    println!("RON: {}", ron::to_string(&players).unwrap());

    let test_map = fs::read_to_string("assets/maps/cell_blocks.map").unwrap();
    let map: game_test::combat_map::CombatMap = ron::from_str(test_map.as_str()).unwrap();

    println!("RON: {}", ron::to_string(&map).unwrap());
}
