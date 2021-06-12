// use ron::ser::{to_string_pretty, PrettyConfig};
// use serde::{Serialize, Deserialize};
// use std::{collections::HashMap, iter::FromIterator};
// use std::fs::File;
// use std::io::prelude::*;

mod game_data;
use game_data::GameData;
// use std::io::Error;

fn main() {
    let file_name = "assets/game.data";
    let game_data = GameData::load_from_file(file_name).expect("Game data not read");
    println!("[{}] [{}] [{}]", game_data.levels[0], game_data.levels[1], game_data.levels[2]);
}
