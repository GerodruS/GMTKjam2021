use macroquad::prelude::*;

mod game_data;
use game_data::GameData;

#[macroquad::main("GMTK Game Jam 2021")]
async fn main() {
    let file_name = "assets/game.data";
    let game_data = GameData::load_from_file(file_name)
        .await
        .expect("Game data not read");

    loop {
        clear_background(BLACK);

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("GMTK Game Jam 2021").show(egui_ctx, |ui| {
                ui.label(format!(
                    "[{}] [{}] [{}]",
                    game_data.levels[0], game_data.levels[1], game_data.levels[2],
                ));
            });
        });

        egui_macroquad::draw();
        next_frame().await
    }
}
