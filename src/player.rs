use std::cmp::{max, min};

use crate::{
    chain::ChainPos,
    cursor::{CursorPos, CursorTile},
    map::Map,
    state::GameState,
};
use bevy::{
    input::{ButtonState, mouse::MouseButtonInput},
    prelude::*,
    sprite::Anchor,
};
use bevy_ecs_tilemap::prelude::*;

const TILE_SIZE: f32 = 32.0;

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct PlayerPos(pub IVec2);

impl Default for PlayerPos {
    fn default() -> Self {
        Self(ivec2(-1000, -1000))
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerPos::default()).add_systems(
            Update,
            (spawn_player, update_player, get_tilepos, fire_chain),
        );
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<GameState>>,
    mut player_pos: ResMut<PlayerPos>,
    map: Res<Map>,
) {
    if state.get() == &GameState::LoadPlayer {
        player_pos.0 = ivec2((map.player_pos.x) as i32, (map.player_pos.y) as i32);
        commands.spawn((
            Sprite {
                image: asset_server.load("player.png"),
                anchor: Anchor::TopLeft,
                ..default()
            },
            Transform::from_xyz(
                player_pos.0.x as f32 * 32.0,
                player_pos.0.y as f32 * -32.0,
                1.0,
            ),
            Player {},
        ));
        *state = State::new(GameState::Gameplay);
    }
}

fn fire_chain(
    cursor_tile: Res<CursorTile>,
    player_pos: Res<PlayerPos>,
    mut mouse_button_event: EventReader<MouseButtonInput>,
    mut chain_query: Query<(&mut Visibility, &mut ChainPos)>,
) {
    if (cursor_tile.0.x == player_pos.0.x && cursor_tile.0.y != player_pos.0.y)
        || (cursor_tile.0.x != player_pos.0.x && cursor_tile.0.y == player_pos.0.y)
    {
        for event in mouse_button_event.read() {
            if event.button == MouseButton::Left && event.state == ButtonState::Pressed {
                if cursor_tile.0.x != player_pos.0.x {
                    let min_x = min(cursor_tile.0.x, player_pos.0.x) + 1;
                    let max_x = max(cursor_tile.0.x, player_pos.0.x) + 1;
                    let mut curr_x = min_x;
                    for (mut vis, mut pos) in chain_query.iter_mut() {
                        *vis = Visibility::Visible;
                        pos.0 = vec2(curr_x as f32, cursor_tile.0.y as f32);
                        curr_x += 1;
                        if curr_x == max_x {
                            break;
                        }
                    }
                } else {
                    let min_y = min(cursor_tile.0.y, player_pos.0.y) + 1;
                    let max_y = max(cursor_tile.0.y, player_pos.0.y) + 1;
                    let mut curr_y = min_y;
                    for (mut vis, mut pos) in chain_query.iter_mut() {
                        *vis = Visibility::Visible;
                        pos.0 = vec2(cursor_tile.0.x as f32, curr_y as f32);
                        curr_y += 1;
                        if curr_y == max_y {
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn get_tilepos(
    cursor_pos: Res<CursorPos>,
    mut cursor_tile: ResMut<CursorTile>,
    tilemap_query: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapTileSize,
        &TilemapType,
        &TileStorage,
        &Transform,
        &TilemapAnchor,
    )>,
) {
    for (map_size, grid_size, tile_size, map_type, _tile_storage, map_transform, anchor) in
        tilemap_query.iter()
    {
        let cursor_pos: Vec2 = cursor_pos.0;
        let cursor_in_map_pos: Vec2 = {
            let cursor_pos = Vec4::from((cursor_pos, 0.0, 1.0));
            (map_transform.compute_matrix().inverse() * cursor_pos).xy()
        };

        if let Some(tile_pos) = TilePos::from_world_pos(
            &cursor_in_map_pos,
            map_size,
            grid_size,
            tile_size,
            map_type,
            anchor,
        ) {
            cursor_tile.0 = ivec2(tile_pos.x as i32, map_size.y as i32 - 1 - tile_pos.y as i32);
        }
    }
}

fn update_player(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    mut map: ResMut<Map>,
) {
    if let Ok(mut transform) = query.single_mut() {
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
            transform.translation = pos;
        }
    }
}
