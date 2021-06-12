use macroquad::prelude::*;

use game_data::GameData;
use game_state::GameState;

use crate::game_data::LevelAdditionalData;

mod game_data;
mod game_state;

#[macroquad::main("GMTK Game Jam 2021")]
async fn main() {
    let file_name = "assets/game.data";
    let game_data = GameData::load_from_file(file_name)
        .await
        .expect("Game data not read");

    let mut game_state = GameState::Start;

    let mut camera = Default::default();
    set_camera(&camera);

    'game_loop: loop {
        clear_background(BLACK);

        match &game_state {
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
                                game_state = GameState::LevelPreparing { level_index: index };
                            }
                        }
                    });
                });
            }

            GameState::LevelPreparing { level_index } => {
                egui_macroquad::ui(|_| {});

                let level_data = &(game_data.levels[*level_index]);
                let layout = level_data.layouts[0].clone();

                let mut width = 0;
                let mut height = 0;
                let mut start_position = -Vec2::ONE;
                let mut finish_position = -Vec2::ONE;
                for line in layout.split(|c| c == '\n' || c == ' ') {
                    let line_width = line.len();
                    if 0 < line_width {
                        width = width.max(line_width);
                        height += 1;
                        match line.chars().position(|c| c == 's') {
                            Some(i) => {
                                if start_position == -Vec2::ONE {
                                    start_position.x = i as f32;
                                    start_position.y = (height - 1) as f32;
                                } else {
                                    // TODO: second start_position -- not good
                                }
                            }
                            _ => {}
                        }
                        match line.chars().position(|c| c == 'f') {
                            Some(i) => {
                                if finish_position == -Vec2::ONE {
                                    finish_position.x = i as f32;
                                    finish_position.y = (height - 1) as f32;
                                } else {
                                    // TODO: second finish_position -- not good
                                }
                            }
                            _ => {}
                        }
                    }
                }
                println!(
                    "w={} h={} s={:} f={:}",
                    width, height, start_position, finish_position
                );

                game_state = GameState::Level {
                    level_index: *level_index,
                    level_add_data: LevelAdditionalData {
                        size: vec2(width as f32, height as f32),
                        start_position,
                        finish_position,
                        points_data: vec![]
                    },
                };
            }

            GameState::Level {
                level_index,
                level_add_data,
            } => {
                let level_data = &(game_data.levels[*level_index]);
                update_screen_size(&mut camera, level_add_data.size);

                draw_rectangle_lines(
                    -0.5,
                    -0.5,
                    level_add_data.size.x,
                    level_add_data.size.y,
                    0.1,
                    GRAY,
                );
                for y in 0..level_add_data.size.y as i32 {
                    for x in 0..level_add_data.size.x as i32 {
                        draw_circle(x as f32, y as f32, 0.1, GRAY);
                    }
                }

                let point_radius = 0.5;
                draw_circle(
                    level_add_data.start_position.x,
                    level_add_data.start_position.y,
                    point_radius,
                    YELLOW,
                );
                draw_circle(
                    level_add_data.finish_position.x,
                    level_add_data.finish_position.y,
                    point_radius,
                    GREEN,
                );

                let mouse_position = mouse_position();
                let mouse_position =
                    camera.screen_to_world(vec2(mouse_position.0, mouse_position.1));
                draw_line(
                    level_add_data.start_position.x,
                    level_add_data.start_position.y,
                    mouse_position.x,
                    mouse_position.y,
                    0.1,
                    RED,
                );

                if is_mouse_button_released(MouseButton::Left) {
                    if mouse_position.distance_squared(level_add_data.finish_position)
                        < point_radius * point_radius
                    {
                        println!("DONE");
                    }
                }

                let mut next_game_state = None;
                egui_macroquad::ui(|egui_ctx| {
                    egui::Window::new("GMTK Game Jam 2021").show(egui_ctx, |ui| {
                        {
                            ui.label(format!(
                                "Playing level: '{}. {}'",
                                level_index + 1,
                                level_data.name
                            ));
                        }
                        if ui.button("Exit to Main Menu").clicked() {
                            next_game_state = Some(GameState::MainMenu);
                        }
                    });
                });

                if let Some(state) = next_game_state {
                    game_state = state;
                }
            }

            GameState::Quit => {
                break 'game_loop;
            }
        };

        egui_macroquad::draw();
        next_frame().await
    }
}

pub fn update_screen_size(camera: &mut Camera2D, virtual_size: Vec2) {
    // TODO: add delay and test for new screen size

    let real_screen_size = vec2(screen_width(), screen_height());

    let real_aspect_ratio = real_screen_size.x / real_screen_size.y;
    let target_aspect_ratio = virtual_size.x / virtual_size.y;
    let virtual_screen_size = if target_aspect_ratio < real_aspect_ratio {
        vec2(real_aspect_ratio * virtual_size.y, virtual_size.y)
    } else {
        vec2(virtual_size.x, virtual_size.x / real_aspect_ratio)
    };

    camera.zoom = vec2(2.0 / virtual_screen_size.x, -2.0 / virtual_screen_size.y);
    camera.target = (vec2(virtual_size.x, virtual_size.y) - Vec2::ONE) / 2.0;
    set_camera(camera);
}
