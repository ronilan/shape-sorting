#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameType {
    Mouse,
    Key,
    Mix,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    Splash,
    Ready,
    Running,
    Paused,
    Completed,
}
