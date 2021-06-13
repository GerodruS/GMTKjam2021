use crate::game_data::LevelAdditionalData;

pub enum GameState {
    Start,
    MainMenu,
    LevelPreparing {
        level_index: usize,
    },
    Level {
        level_index: usize,
        layout_index: usize,
    },
    Quit,
}
