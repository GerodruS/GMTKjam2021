use macroquad::prelude::*;

mod game_data;
use game_data::GameData;

#[macroquad::main("GMTK Game Jam 2021")]
async fn main() {
    let file_name = "assets/game.data";
    let game_data = GameData::load_from_file(file_name).expect("Game data not read");
    println!("[{}] [{}] [{}]", game_data.levels[0], game_data.levels[1], game_data.levels[2]);

    loop {
        clear_background(BLACK);
        draw_text("GMTK Game Jam 2021", 20.0, 20.0, 30.0, WHITE);
        next_frame().await
    }
}
