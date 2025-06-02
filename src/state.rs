use bevy::prelude::*;

#[derive(States, Default, Hash, Clone, Copy, Eq, PartialEq, Debug)]
pub enum GameState {
    #[default]
    LoadMap = 0,
    LoadPlayer = 1,
    Gameplay = 2,
}
