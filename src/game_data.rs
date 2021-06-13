use std::fs::File;

use macroquad::prelude::*;
use parry2d::math::{Isometry, Real};
use parry2d::shape::{Ball, Segment};
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct GameData {
    pub resolution: (f32, f32),
    pub levels: Vec<LevelData>,
}

impl GameData {
    pub async fn load_from_file(file_name: &str) -> Result<GameData, &str> {
        let file_result = load_file(file_name).await;
        match file_result {
            Ok(file) => {
                let game_data: GameData =
                    ron::de::from_bytes(file.as_ref()).expect("Deserialization from file failed");
                Ok(game_data)
            }
            Err(_) => {
                let pretty_config = PrettyConfig::new().with_separate_tuple_members(true);
                // TODO: do not use FILE at WebGL version
                let new_file = File::create(file_name).expect("file not created");
                let new_game_data = GameData::default();
                ron::ser::to_writer_pretty(new_file, &new_game_data, pretty_config)
                    .expect("Serialization to new file failed");
                Ok(new_game_data)
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LevelData {
    pub name: String,
    pub win_count: usize,
    pub layouts: Vec<String>,
}

pub struct LevelAdditionalData {
    pub size: Vec2,
    pub points_data: Vec<PointData>,
    pub obstacles_data: Vec<ObstacleData>,
    pub start_point_index: usize,
    pub finish_point_index: usize,
}

pub struct PointData {
    pub position: Vec2,
    pub point_type: PointType,
}

pub enum PointType {
    Start,
    Finish,
    Common { pair_index: usize },
}

pub struct ConnectionData {
    pub from_index: usize,
    pub to_index: usize,
    pub segment: Segment, // for collision detection
}

pub struct ObstacleData {
    pub position: Vec2,
    pub radius: f32,
    pub ball: Ball, // for collision detection
}

impl ObstacleData {
    pub fn get_isometry(&self) -> Isometry<Real> {
        Isometry::translation(self.position.x, self.position.y)
    }
}
