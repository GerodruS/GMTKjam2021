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
                ui.label("Select level:");
                for (index, level_data) in game_data.levels.iter().enumerate() {
                    if ui
                        .button(format!("{}. {}", index + 1, level_data.name))
                        .clicked()
                    {
                        println!("Lvl: '{}'", level_data.name);
                    }
                }
            });
        });

        egui_macroquad::draw();
        next_frame().await
    }
}
