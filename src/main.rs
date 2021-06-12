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

    let mut camera = Camera2D {
        target: Vec2::ZERO,
        ..Default::default()
    };
    set_camera(&camera);

    'game_loop: loop {
        clear_background(BLACK);
        update_screen_size(&mut camera, &game_data);

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
                let level_data = &(game_data.levels[level_index]);
                let layout = level_data.layouts[0].clone();
                // let layout = layout.trim_matches(|c| c == '\n');
                // let layout = layout.replace(" ", "");
                // println!("s\n[{}]\n[{}]", level_data.layouts[0].as_str(), layout);


                egui_macroquad::ui(|egui_ctx| {
                    egui::Window::new("GMTK Game Jam 2021").show(egui_ctx, |ui| {
                        {
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

pub fn update_screen_size(camera: &mut Camera2D, game_data: &GameData) {
    // TODO: add delay and test for new screen size

    let real_screen_size = vec2(screen_width(), screen_height());

    let real_aspect_ratio = real_screen_size.x / real_screen_size.y;
    let target_aspect_ratio = game_data.resolution.0 / game_data.resolution.1;
    let virtual_screen_size = if target_aspect_ratio < real_aspect_ratio {
        vec2(
            real_aspect_ratio * game_data.resolution.1,
            game_data.resolution.1,
        )
    } else {
        vec2(
            game_data.resolution.0,
            game_data.resolution.0 / real_aspect_ratio,
        )
    };

    camera.zoom = vec2(2.0 / virtual_screen_size.x, -2.0 / virtual_screen_size.y);
    set_camera(camera);
}
