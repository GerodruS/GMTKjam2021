pub enum GameState {
    Start,
    MainMenu,
    Level {
        level_index: usize,
    },
    Quit,
}
