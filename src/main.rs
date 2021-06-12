use macroquad::prelude::*;

use game_data::GameData;
use game_state::GameState;

use crate::game_data::PointType::{Common, Start, Finish};
use crate::game_data::{ConnectionData, LevelAdditionalData, PointData};

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

    // TODO: move under GameState::Level
    let mut connections_data = Vec::<ConnectionData>::new();
    //

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
                let mut points_data = Vec::new();
                let mut pair_ids = Vec::new();
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
                        for (i, char) in line.chars().enumerate() {
                            if char.is_digit(10) {
                                points_data.push(PointData {
                                    position: vec2(i as f32, (height - 1) as f32),
                                    point_type: Common {
                                        pair_index: 0, // will be filled later
                                    },
                                });
                                pair_ids.push(char);
                            }
                        }
                    }
                }

                for (i, point_data) in points_data.iter_mut().enumerate() {
                    let pair_id = pair_ids[i];
                    let mut another_point_index = None;
                    for (j, another_pair_id) in pair_ids.iter().enumerate() {
                        if i != j && pair_id == *another_pair_id {
                            if another_point_index == None {
                                another_point_index = Some(j);
                            } else {
                                // TODO: not good -- more than two points has one id
                            }
                        }
                    }
                    if let Some(index) = another_point_index {
                        point_data.point_type = Common { pair_index: index };
                    } else {
                        // TODO: not good -- only one point has this id
                    }
                }

                let start_point_index = points_data.len();
                points_data.push(PointData {
                    position: start_position,
                    point_type: Start,
                });

                let finish_point_index = points_data.len();
                points_data.push(PointData {
                    position: finish_position,
                    point_type: Finish,
                });

                println!(
                    "w={} h={} s={:} f={:}",
                    width, height, start_position, finish_position
                );

                connections_data.clear();
                game_state = GameState::Level {
                    level_index: *level_index,
                    level_add_data: LevelAdditionalData {
                        size: vec2(width as f32, height as f32),
                        points_data,
                        start_point_index,
                        finish_point_index,
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
                        draw_circle(x as f32, y as f32, 0.1, DARKGRAY);
                    }
                }

                let point_radius = 0.25;
                for point_data in &level_add_data.points_data {
                    draw_circle(
                        point_data.position.x,
                        point_data.position.y,
                        point_radius,
                        ORANGE,
                    );
                }
                // // debug pairs
                // for point_data in &level_add_data.points_data {
                //     draw_line(
                //         point_data.position.x,
                //         point_data.position.y,
                //         level_add_data.points_data[point_data.pair_index].position.x,
                //         level_add_data.points_data[point_data.pair_index].position.y,
                //         0.01,
                //         GREEN,
                //     );
                // }
                draw_circle(
                    level_add_data.points_data[level_add_data.start_point_index]
                        .position
                        .x,
                    level_add_data.points_data[level_add_data.start_point_index]
                        .position
                        .y,
                    point_radius,
                    YELLOW,
                );
                draw_circle(
                    level_add_data.points_data[level_add_data.finish_point_index]
                        .position
                        .x,
                    level_add_data.points_data[level_add_data.finish_point_index]
                        .position
                        .y,
                    point_radius,
                    GREEN,
                );

                let current_start = if let Some(connection_data) = connections_data.last() {
                    let to_index = connection_data.to_index;
                    let point_data = &level_add_data.points_data[to_index];
                    if let Common { pair_index } = point_data.point_type {
                        Some((pair_index, level_add_data.points_data[pair_index].position))
                    } else {
                        None
                    }
                } else {
                    Some((
                        level_add_data.start_point_index,
                        level_add_data.points_data[level_add_data.start_point_index].position,
                    ))
                };

                let mouse_position = mouse_position();
                let mouse_position =
                    camera.screen_to_world(vec2(mouse_position.0, mouse_position.1));

                if let Some((current_start_index, _)) = current_start {
                    if is_mouse_button_released(MouseButton::Left) {
                        if mouse_position.distance_squared(
                            level_add_data.points_data[level_add_data.finish_point_index].position,
                        ) < point_radius * point_radius
                        {
                            connections_data.push(ConnectionData {
                                from_index: current_start_index,
                                to_index: level_add_data.finish_point_index,
                            });
                        } else {
                            for (i, point_data) in level_add_data.points_data.iter().enumerate() {
                                if mouse_position.distance_squared(point_data.position)
                                    < point_radius * point_radius
                                {
                                    if !connections_data
                                        .iter()
                                        .any(|elem| elem.from_index == i || elem.to_index == i)
                                    {
                                        connections_data.push(ConnectionData {
                                            from_index: current_start_index,
                                            to_index: i,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }

                let is_win = {
                    if let Some(connection_data) = connections_data.last() {
                        connection_data.to_index == level_add_data.finish_point_index
                    } else {
                        false
                    }
                };

                for connection_data in &connections_data {
                    let from_position =
                        level_add_data.points_data[connection_data.from_index].position;
                    let to_position = level_add_data.points_data[connection_data.to_index].position;
                    draw_line(
                        from_position.x,
                        from_position.y,
                        to_position.x,
                        to_position.y,
                        0.1,
                        VIOLET,
                    );
                }

                if let Some((_, current_start_position)) = current_start {
                    draw_line(
                        current_start_position.x,
                        current_start_position.y,
                        mouse_position.x,
                        mouse_position.y,
                        0.1,
                        RED,
                    );
                }

                let mut next_game_state = None;
                egui_macroquad::ui(|egui_ctx| {
                    egui::Window::new("GMTK Game Jam 2021").show(egui_ctx, |ui| {
                        ui.label(format!(
                            "Playing level: '{}. {}'",
                            level_index + 1,
                            level_data.name
                        ));
                        if ui.button("Exit to Main Menu").clicked() {
                            next_game_state = Some(GameState::MainMenu);
                        }
                    });
                    if is_win {
                        egui::Window::new("Win!").show(egui_ctx, |ui| {
                            ui.label("Great Success!");
                            if ui.button("Exit to Main Menu").clicked() {
                                next_game_state = Some(GameState::MainMenu);
                            }
                        });
                    }
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
