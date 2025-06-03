use std::cmp::{max, min};

use crate::{
    chain::ChainPos,
    cursor::{CursorPos, CursorTile},
    map::{CurrentMap, PlayerSpawn},
    state::GameState,
};
use bevy::{
    input::{ButtonState, mouse::MouseButtonInput},
    prelude::*,
    sprite::Anchor,
};
use bevy_ecs_tiled::prelude::{TiledMap, from_tiled_position_to_world_space};
use bevy_ecs_tilemap::prelude::*;

const TILE_SIZE: f32 = 32.0;

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct PlayerPos(pub IVec2);

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

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
            (
                spawn_player,
                update_player,
                get_tilepos,
                fire_chain,
                animate_sprite,
            ),
        );
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<GameState>>,
    mut player_pos: ResMut<PlayerPos>,
    player_spawn_query: Query<&PlayerSpawn>,
    tiled_assets: Res<Assets<TiledMap>>,
    current_map: Res<CurrentMap>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if state.get() == &GameState::LoadPlayer {
        if let Some(player) = player_spawn_query.iter().next() {
            let texture = asset_server.load("sprites/newplayer.png");
            let layout = TextureAtlasLayout::from_grid(uvec2(32, 32), 5, 1, None, None);
            let texture_atlas_layout = texture_atlas_layouts.add(layout);
            let animation_indicies = AnimationIndices { first: 0, last: 4 };
            let tiled_map = current_map.0.clone().unwrap();
            let tiled_map = tiled_assets.get(&tiled_map.clone_weak()).unwrap();
            let position = from_tiled_position_to_world_space(
                tiled_map,
                &TilemapAnchor::TopLeft,
                player.position * TILE_SIZE,
            );
            let mut sprite = Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indicies.first,
                },
            );
            sprite.anchor = Anchor::TopLeft;
            player_pos.0 = ivec2(player.position.x as i32, player.position.y as i32);
            commands.spawn((
                sprite,
                Transform::from_xyz(position.x as f32, position.y as f32, 1.0),
                Anchor::TopLeft,
                Player {},
                AnimationIndices { first: 0, last: 4 },
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            ));
            *state = State::new(GameState::Gameplay);
        }
    }
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
    state: Res<State<GameState>>,
) {
    if state.get() == &GameState::Gameplay {
        for (indicies, mut timer, mut sprite) in &mut query {
            timer.tick(time.delta());

            if timer.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = if atlas.index == indicies.last {
                        indicies.first
                    } else {
                        atlas.index + 1
                    }
                }
            }
        }
    }
}

fn fire_chain(
    cursor_tile: Res<CursorTile>,
    player_pos: Res<PlayerPos>,
    mut mouse_button_event: EventReader<MouseButtonInput>,
    mut chain_query: Query<(&mut Visibility, &mut ChainPos)>,
    state: Res<State<GameState>>,
) {
    if state.get() == &GameState::Gameplay
        && ((cursor_tile.0.x == player_pos.0.x && cursor_tile.0.y != player_pos.0.y)
            || (cursor_tile.0.x != player_pos.0.x && cursor_tile.0.y == player_pos.0.y))
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

fn update_player(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Transform, With<Player>>) {
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

        // if map.move_player(delta.x, delta.y) {
        //     let mut pos = map.player_pos.extend(1.0);
        //     pos.x *= TILE_SIZE;
        //     // negative as we are moving downwards
        //     pos.y *= -TILE_SIZE;
        //     transform.translation = pos;
        // }
    }
}
