use bevy::{prelude::*, sprite::Anchor};

use crate::{map::Map, state::GameState};

const TILE_SIZE: f32 = 32.0;

#[derive(Component)]
struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_player, update_player));
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<GameState>>,
    map_query: Query<&Map>,
) {
    if state.get() == &GameState::LoadPlayer {
        if let Ok(map) = map_query.single() {
            commands.spawn((
                Sprite {
                    image: asset_server.load("player.png"),
                    anchor: Anchor::TopLeft,
                    ..default()
                },
                Transform::from_xyz(map.player_pos.x * 32.0, map.player_pos.y * -32.0, 1.0),
                Player {},
            ));
            *state = State::new(GameState::Gameplay);
        }
    }
}

fn update_player(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
    mut map_query: Query<&mut Map>,
) {
    if let Ok(mut transform) = query.single_mut() {
        if let Ok(mut map) = map_query.single_mut() {
            let mut delta = IVec2::splat(0);
            if input.just_pressed(KeyCode::KeyA) {
                delta.x -= 1;
            }
            if input.just_pressed(KeyCode::KeyD) {
                delta.x += 1;
            }
            if input.just_pressed(KeyCode::KeyW) {
                delta.y -= 1;
            }
            if input.just_pressed(KeyCode::KeyS) {
                delta.y += 1;
            }

            if map.move_player(delta.x, delta.y) {
                let mut pos = map.player_pos.extend(1.0);
                pos.x *= TILE_SIZE;
                // negative as we are moving downwards
                pos.y *= -TILE_SIZE;
                transform.1.translation = pos;
            }
        }
    }
}
