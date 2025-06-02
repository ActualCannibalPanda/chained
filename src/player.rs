use crate::{
    chain::ChainPos,
    cursor::{CursorPos, CursorTile},
    map::Map,
    state::GameState,
};
use bevy::{prelude::*, sprite::Anchor};
use bevy_ecs_tilemap::prelude::*;

const TILE_SIZE: f32 = 32.0;

#[derive(Component)]
struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_player, update_player, get_tilepos, fire_chain),
        );
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<GameState>>,
    map: Res<Map>,
) {
    if state.get() == &GameState::LoadPlayer {
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

fn fire_chain(
    _cursor_tile: Res<CursorTile>,
    mut chain_query: Query<&mut Visibility, With<ChainPos>>,
) {
    for mut vis in chain_query.iter_mut() {
        *vis = Visibility::Visible
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
            cursor_tile.0 = ivec2(tile_pos.x as i32, tile_pos.y as i32);
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
