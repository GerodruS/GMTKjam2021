use macroquad::prelude::*;
use parry2d::math::Isometry;
use parry2d::na::{Point2, Vector2};
use parry2d::query::{Ray, RayCast};
use parry2d::shape::{Ball, Segment};

use game_data::GameData;
use game_state::GameState;

use crate::game_data::PointType::{Common, Finish, Start};
use crate::game_data::{
    ConnectionData, LayoutAdditionalData, LevelAdditionalData, ObstacleData, PointData, PointId,
    PointType,
};

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
    let mut level_additional_data = LevelAdditionalData {
        layouts_data: vec![],
    };
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
                let mut start_layout_index = 0;
                let mut layouts_data =
                    Vec::<LayoutAdditionalData>::with_capacity(level_data.layouts.len());
                let mut pair_ids = Vec::new();
                for (layout_index, layout) in level_data.layouts.iter().enumerate() {
                    let mut start_position = None;
                    let mut finish_position = None;
                    let mut points_data = Vec::new();
                    let mut obstacles_data = Vec::new();
                    let mut layout_width = 0;
                    let mut layout_height = 0;
                    for line in layout.split(|c| c == '\n' || c == ' ') {
                        let line_width = line.len();
                        if 0 < line_width {
                            layout_width = layout_width.max(line_width);
                            layout_height += 1;
                            match line.chars().position(|c| c == 's') {
                                Some(i) => {
                                    if start_position == None {
                                        start_position =
                                            Some(vec2(i as f32, (layout_height - 1) as f32));
                                    } else {
                                        // TODO: second start_position -- not good
                                    }
                                }
                                _ => {}
                            }
                            match line.chars().position(|c| c == 'f') {
                                Some(i) => {
                                    if finish_position == None {
                                        finish_position =
                                            Some(vec2(i as f32, (layout_height - 1) as f32));
                                    } else {
                                        // TODO: second finish_position -- not good
                                    }
                                }
                                _ => {}
                            }
                            for (i, char) in line.chars().enumerate() {
                                if char.is_digit(10) {
                                    let point_index = points_data.len();
                                    points_data.push(PointData {
                                        position: vec2(i as f32, (layout_height - 1) as f32),
                                        point_type: Common {
                                            layout_index: 0, // will be filled later
                                            pair_index: 0,   // will be filled later
                                        },
                                    });
                                    pair_ids.push((
                                        char,
                                        PointId {
                                            layout_index,
                                            point_index,
                                        },
                                    ));
                                }
                                if char == 'z' {
                                    let radius = 0.5; // TODO: move radius to constants
                                    obstacles_data.push(ObstacleData {
                                        position: vec2(i as f32, (layout_height - 1) as f32),
                                        radius,
                                        ball: Ball::new(radius),
                                    })
                                }
                            }
                        }
                    }

                    let start_point_index = if let Some(position) = start_position {
                        points_data.push(PointData {
                            position,
                            point_type: Start,
                        });
                        Some(points_data.len() - 1)
                    } else {
                        None
                    };

                    let finish_point_index = if let Some(position) = finish_position {
                        points_data.push(PointData {
                            position,
                            point_type: Finish,
                        });
                        Some(points_data.len() - 1)
                    } else {
                        None
                    };

                    layouts_data.push(LayoutAdditionalData {
                        size: vec2(layout_width as f32, layout_height as f32),
                        points_data,
                        obstacles_data,
                        start_point_index,
                        finish_point_index,
                    });

                    if start_point_index != None {
                        start_layout_index = layout_index;
                    }
                }

                let mut pair_ids_index = 0;
                for (layout_index, _) in level_data.layouts.iter().enumerate() {
                    let layout_data = &mut layouts_data[layout_index];
                    for (_, point_data) in layout_data.points_data.iter_mut().enumerate()
                    {
                        match point_data.point_type {
                            PointType::Common { .. } => {
                                pair_ids_index += 1;
                                let pair_ids_index = pair_ids_index - 1;
                                let pair_id = &pair_ids[pair_ids_index];
                                let mut another_point_index = None;
                                for (j, another_pair_id) in pair_ids.iter().enumerate() {
                                    if pair_ids_index != j && pair_id.0 == another_pair_id.0 {
                                        if another_point_index == None {
                                            another_point_index = Some(another_pair_id);
                                        } else {
                                            // TODO: not good -- more than two points has one id
                                        }
                                    }
                                }
                                if let Some(another_pair_id) = another_point_index {
                                    point_data.point_type = Common {
                                        layout_index: another_pair_id.1.layout_index,
                                        pair_index: another_pair_id.1.point_index,
                                    };
                                } else {
                                    // TODO: not good -- only one point has this id
                                }
                            }
                            _ => {}
                        }
                    }
                }

                connections_data.clear();
                level_additional_data = LevelAdditionalData { layouts_data };
                game_state = GameState::Level {
                    level_index: *level_index,
                    layout_index: start_layout_index,
                };
            }

            GameState::Level {
                level_index,
                layout_index,
            } => {
                let mut next_game_state = None;
                let level_add_data = &level_additional_data;
                let level_data = &(game_data.levels[*level_index]);
                let layout_data = &level_add_data.layouts_data[*layout_index];
                update_screen_size(&mut camera, layout_data.size);

                draw_rectangle_lines(
                    -0.5,
                    -0.5,
                    layout_data.size.x,
                    layout_data.size.y,
                    0.1,
                    GRAY,
                );
                for y in 0..layout_data.size.y as i32 {
                    for x in 0..layout_data.size.x as i32 {
                        draw_circle(x as f32, y as f32, 0.1, DARKGRAY);
                    }
                }

                let point_radius = 0.25;
                for point_data in &layout_data.points_data {
                    draw_circle(
                        point_data.position.x,
                        point_data.position.y,
                        point_radius,
                        ORANGE,
                    );
                }
                for obstacle_data in &layout_data.obstacles_data {
                    draw_circle(
                        obstacle_data.position.x,
                        obstacle_data.position.y,
                        obstacle_data.radius,
                        BLUE,
                    );
                }

                if let Some(start_point_index) = layout_data.start_point_index {
                    draw_circle(
                        layout_data.points_data[start_point_index].position.x,
                        layout_data.points_data[start_point_index].position.y,
                        point_radius,
                        YELLOW,
                    );
                }

                if let Some(finish_point_index) = layout_data.finish_point_index {
                    draw_circle(
                        layout_data.points_data[finish_point_index].position.x,
                        layout_data.points_data[finish_point_index].position.y,
                        point_radius,
                        GREEN,
                    );
                }

                let current_start = if let Some(connection_data) = connections_data.last() {
                    let point_data = &level_add_data.layouts_data[connection_data.layout_index]
                        .points_data[connection_data.to_point_index];
                    if let Common {
                        layout_index: _,
                        pair_index,
                    } = point_data.point_type
                    {
                        Some((pair_index, layout_data.points_data[pair_index].position))
                    } else {
                        None
                    }
                } else if let Some(start_point_index) = layout_data.start_point_index {
                    Some((
                        start_point_index,
                        layout_data.points_data[start_point_index].position,
                    ))
                } else {
                    panic!("Can not find start!");
                };

                let mouse_position = mouse_position();
                let mouse_position =
                    camera.screen_to_world(vec2(mouse_position.0, mouse_position.1));

                let intersection_point = {
                    let mut intersection_point = None;
                    if let Some((_, current_start_position)) = current_start {
                        let vector = mouse_position - current_start_position;
                        let ray = Ray::new(
                            Point2::new(current_start_position.x, current_start_position.y),
                            Vector2::new(vector.x, vector.y),
                        );
                        let mut has_intersection = false;
                        let mut min_time: f32 = 1.0;
                        for connection_data in &connections_data {
                            if connection_data.layout_index == *layout_index {
                                let segment = connection_data.segment;
                                if let Some(time) = segment.cast_ray(
                                    &Isometry::identity(),
                                    &ray,
                                    vector.length(),
                                    true,
                                ) {
                                    if time < 1.0 {
                                        println!("1 t={}", time);
                                        has_intersection = true;
                                        min_time = min_time.min(time);
                                    }
                                }
                            }
                        }
                        for obstacle_data in &layout_data.obstacles_data {
                            let ball = obstacle_data.ball;
                            let isometry = obstacle_data.get_isometry();
                            if let Some(time) =
                                ball.cast_ray(&isometry, &ray, vector.length(), true)
                            {
                                if time < 1.0 {
                                    println!("2 t={}", time);
                                    has_intersection = true;
                                    min_time = min_time.min(time);
                                }
                            }
                        }
                        if has_intersection {
                            let vector = vector * min_time;
                            intersection_point = Some(current_start_position + vector);
                        }
                    }
                    intersection_point
                };

                let mut next_layout_index = None;
                if let Some((current_start_index, _)) = current_start {
                    if is_mouse_button_pressed(MouseButton::Left) {
                        let mut index = None;
                        for (i, connection_data) in connections_data.iter().enumerate() {
                            if connection_data.layout_index == *layout_index {
                                let point_data = &level_add_data.layouts_data
                                    [connection_data.layout_index]
                                    .points_data[connection_data.from_point_index];
                                if mouse_position.distance_squared(point_data.position)
                                    < point_radius * point_radius
                                {
                                    index = Some(i);
                                    break;
                                }
                            }

                            let point_data = &level_add_data.layouts_data
                                [connection_data.layout_index]
                                .points_data[connection_data.to_point_index];
                            if let Common {
                                layout_index: pair_layout_index,
                                pair_index,
                            } = point_data.point_type
                            {
                                if pair_layout_index == *layout_index {
                                    let point_data = &level_add_data.layouts_data
                                        [pair_layout_index]
                                        .points_data[pair_index];
                                    if mouse_position.distance_squared(point_data.position)
                                        < point_radius * point_radius
                                    {
                                        index = Some(i);
                                        break;
                                    }
                                }
                            }
                        }
                        if let Some(index) = index {
                            connections_data.truncate(index);
                            if let Some(connection_data) = connections_data.last() {
                                let last_point_data = &level_add_data.layouts_data
                                    [connection_data.layout_index]
                                    .points_data[connection_data.to_point_index];
                                if let Common {
                                    layout_index: pair_layout_index,
                                    pair_index: _,
                                } = last_point_data.point_type
                                {
                                    next_game_state = Some(GameState::Level {
                                        level_index: *level_index,
                                        layout_index: pair_layout_index,
                                    });
                                }
                            } else {
                                for (another_layout_index, layout_data) in
                                    level_add_data.layouts_data.iter().enumerate()
                                {
                                    if layout_data.start_point_index != None
                                        && another_layout_index != *layout_index
                                    {
                                        next_game_state = Some(GameState::Level {
                                            level_index: *level_index,
                                            layout_index: another_layout_index,
                                        });
                                    }
                                }
                            }
                        }
                    }

                    if is_mouse_button_released(MouseButton::Left) && intersection_point == None {
                        if let Some(finish_point_index) = layout_data.finish_point_index {
                            if mouse_position.distance_squared(
                                layout_data.points_data[finish_point_index].position,
                            ) < point_radius * point_radius
                            {
                                if level_data.win_count <= connections_data.len() {
                                    let from_position =
                                        layout_data.points_data[current_start_index].position;
                                    let to_position =
                                        layout_data.points_data[finish_point_index].position;
                                    connections_data.push(ConnectionData {
                                        layout_index: *layout_index,
                                        from_point_index: current_start_index,
                                        to_point_index: finish_point_index,
                                        segment: Segment::new(
                                            Point2::new(from_position.x, from_position.y),
                                            Point2::new(to_position.x, to_position.y),
                                        ),
                                    });
                                    if let Common {
                                        layout_index: pair_layout_index,
                                        pair_index: _,
                                    } = layout_data.points_data[finish_point_index].point_type
                                    {
                                        if pair_layout_index != *layout_index {
                                            next_layout_index = Some(pair_layout_index);
                                        }
                                    }
                                } else {
                                    println!(
                                        "not enough {}/{}",
                                        connections_data.len(),
                                        level_data.win_count
                                    );
                                }
                            }
                        }
                        for (i, point_data) in layout_data.points_data.iter().enumerate() {
                            if i != current_start_index
                                && mouse_position.distance_squared(point_data.position)
                                    < point_radius * point_radius
                            {
                                if !connections_data.iter().any(|elem| {
                                    elem.layout_index == *layout_index
                                        && elem.from_point_index == i
                                        || elem.to_point_index == i
                                }) {
                                    let from_position =
                                        layout_data.points_data[current_start_index].position;
                                    let to_position = layout_data.points_data[i].position;
                                    connections_data.push(ConnectionData {
                                        layout_index: *layout_index,
                                        from_point_index: current_start_index,
                                        to_point_index: i,
                                        segment: Segment::new(
                                            Point2::new(from_position.x, from_position.y),
                                            Point2::new(to_position.x, to_position.y),
                                        ),
                                    });
                                    if let Common {
                                        layout_index: pair_layout_index,
                                        pair_index: _,
                                    } = layout_data.points_data[i].point_type
                                    {
                                        if pair_layout_index != *layout_index {
                                            next_layout_index = Some(pair_layout_index);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                if let Some(next_layout_index) = next_layout_index {
                    next_game_state = Some(GameState::Level {
                        level_index: *level_index,
                        layout_index: next_layout_index,
                    });
                }

                let is_win = {
                    if let Some(connection_data) = connections_data.last() {
                        let last_point = &level_add_data.layouts_data[connection_data.layout_index]
                            .points_data[connection_data.to_point_index];
                        last_point.point_type == Finish
                    } else {
                        false
                    }
                };

                for connection_data in &connections_data {
                    if connection_data.layout_index == *layout_index {
                        let from_position =
                            layout_data.points_data[connection_data.from_point_index].position;
                        let to_position =
                            layout_data.points_data[connection_data.to_point_index].position;
                        draw_line(
                            from_position.x,
                            from_position.y,
                            to_position.x,
                            to_position.y,
                            0.1,
                            VIOLET,
                        );
                    }
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

                    if let Some(intersection_point) = intersection_point {
                        draw_line(
                            intersection_point.x,
                            intersection_point.y,
                            mouse_position.x,
                            mouse_position.y,
                            0.1,
                            DARKBLUE,
                        );
                    }
                }

                egui_macroquad::ui(|egui_ctx| {
                    egui::Window::new("GMTK Game Jam 2021").show(egui_ctx, |ui| {
                        ui.label(format!(
                            "Playing level: '{}. {}'\nProgress:{}/{}",
                            level_index + 1,
                            level_data.name,
                            connections_data.len().min(level_data.win_count),
                            level_data.win_count,
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
