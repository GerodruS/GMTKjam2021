use macroquad::prelude::*;

use game_data::GameData;
use game_state::GameState;

mod game_data;
mod game_state;

#[macroquad::main("GMTK Game Jam 2021")]
async fn main() {
    let file_name = "assets/game.data";
    let game_data = GameData::load_from_file(file_name)
        .await
        .expect("Game data not read");

    let mut game_state = GameState::Start;

    'game_loop: loop {
        clear_background(BLACK);

        match game_state {
            GameState::Start => {
                game_state = GameState::MainMenu {};
                egui_macroquad::ui(|_| {});
            }

            GameState::MainMenu {} => {
                egui_macroquad::ui(|egui_ctx| {
                    egui::Window::new("GMTK Game Jam 2021").show(egui_ctx, |ui| {
                        ui.label("Select level:");
                        for (index, level_data) in game_data.levels.iter().enumerate() {
                            if ui
                                .button(format!("{}. {}", index + 1, level_data.name))
                                .clicked()
                            {
                                game_state = GameState::Level {
                                    level_index: index,
                                };
                            }
                        }
                    });
                });
            }

            GameState::Level { level_index } => {
                egui_macroquad::ui(|egui_ctx| {
                    egui::Window::new("GMTK Game Jam 2021").show(egui_ctx, |ui| {
                        {
                            let level_data = &(game_data.levels[level_index]);
                            ui.label(format!("Playing level: '{}. {}'", level_index + 1, level_data.name));
                        }
                        if ui
                            .button("Exit to Main Menu")
                            .clicked()
                        {
                            game_state = GameState::MainMenu {};
                        }
                    });
                });
            }

            GameState::Quit => {
                break 'game_loop;
            }
        };

        egui_macroquad::draw();
        next_frame().await
    }
}
