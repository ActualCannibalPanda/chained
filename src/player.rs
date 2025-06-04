use std::cmp::{max, min};

use crate::{
    chain::ChainPos,
    cursor::{CursorPos, CursorTile},
    map::{CurrentMap, MapTile, PlayerSpawn},
    state::GameState,
};
use bevy::{
    input::{ButtonState, mouse::MouseButtonInput},
    prelude::*,
    sprite::Anchor,
};
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::map::{TilemapTileSize, TilemapType};

const TILE_SIZE: f32 = 32.0;

#[derive(Component)]
struct Player;

#[derive(Resource, Deref, DerefMut)]
struct PlayerPos(pub Vec2);

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

impl Default for PlayerPos {
    fn default() -> Self {
        Self(vec2(-1000.0, -1000.0))
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
            player_pos.0 = vec2(player.position.x, player.position.y);
            println!("{:?}", player_pos.0);
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
    mut chain_query: Query<(&mut Visibility, &mut Transform, &mut ChainPos)>,
    state: Res<State<GameState>>,
) {
    if state.get() == &GameState::Gameplay
        && ((cursor_tile.0.x == player_pos.0.x && cursor_tile.0.y != player_pos.0.y)
            || (cursor_tile.0.x != player_pos.0.x && cursor_tile.0.y == player_pos.0.y))
    {
        for event in mouse_button_event.read() {
            if event.button == MouseButton::Left && event.state == ButtonState::Pressed {
                if cursor_tile.0.x != player_pos.0.x {
                    let dir = if player_pos.0.x < cursor_tile.0.x {
                        1.0
                    } else {
                        -1.0
                    };
                    let min_x = min(cursor_tile.0.x as i32, player_pos.0.x as i32) as f32
                        + if dir < 0.0 { 0.0 } else { 1.0 };
                    let max_x = max(cursor_tile.0.x as i32, player_pos.0.x as i32) as f32
                        + if dir < 0.0 { 0.0 } else { 1.0 };
                    let mut curr_x = min_x;
                    for (mut vis, mut transform, mut pos) in chain_query.iter_mut() {
                        if curr_x == max_x {
                            *vis = Visibility::Hidden;
                            continue;
                        }
                        *vis = Visibility::Visible;
                        pos.0 = vec2(curr_x, cursor_tile.0.y);
                        curr_x += 1.0;
                        transform.rotation = Quat::IDENTITY;
                    }
                } else {
                    let dir = if player_pos.0.y < cursor_tile.0.y {
                        -1.0
                    } else {
                        1.0
                    };
                    let min_y = min(cursor_tile.0.y as i32, player_pos.0.y as i32) as f32
                        + if dir < 0.0 { 0.0 } else { 1.0 };
                    let max_y = max(cursor_tile.0.y as i32, player_pos.0.y as i32) as f32
                        + if dir < 0.0 { 0.0 } else { 1.0 };
                    let mut curr_y = min_y;
                    for (mut vis, mut transform, mut pos) in chain_query.iter_mut() {
                        if curr_y == max_y {
                            *vis = Visibility::Hidden;
                            continue;
                        }
                        *vis = Visibility::Visible;
                        pos.0 = vec2(cursor_tile.0.x, curr_y);
                        curr_y += 1.0;
                        transform.rotation = Quat::IDENTITY;
                        let copy_transform = *transform;
                        transform.rotate_around(
                            vec3(
                                copy_transform.translation.x + 16.0,
                                copy_transform.translation.y + 16.0,
                                0.0,
                            ),
                            Quat::from_euler(EulerRot::YXZ, 0.0, 0.0, 90.0_f64.to_radians() as f32),
                        );
                    }
                }
            }
        }
    }
}

fn get_tilepos(cursor_pos: Res<CursorPos>, mut cursor_tile: ResMut<CursorTile>) {
    let cursor_pos: Vec2 = cursor_pos.0;

    if let Some(tile_pos) = TilePos::from_world_pos(
        &cursor_pos,
        &TilemapSize::new(15, 15),
        &TilemapGridSize::new(32.0, 32.0),
        &TilemapTileSize::new(32.0, 23.0),
        &TilemapType::Square,
        &TilemapAnchor::TopLeft,
    ) {
        cursor_tile.0 = vec2(tile_pos.x as f32, 14.0 - tile_pos.y as f32);
    }
}

fn update_player(
    input: Res<ButtonInput<KeyCode>>,
    map: Res<CurrentMap>,
    tiled_assets: Res<Assets<TiledMap>>,
    mut player_pos: ResMut<PlayerPos>,
    map_query: Query<(&MapTile, &TilePos)>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let mut delta = Vec2::ZERO;
    if input.just_pressed(KeyCode::KeyA) {
        delta.x -= 1.0;
    }
    if input.just_pressed(KeyCode::KeyD) {
        delta.x += 1.0;
    }
    if input.just_pressed(KeyCode::KeyW) {
        delta.y -= 1.0;
    }
    if input.just_pressed(KeyCode::KeyS) {
        delta.y += 1.0;
    }
    if delta != Vec2::ZERO {
        if let Ok(mut player) = player_query.single_mut() {
            let target_tile = vec2(14.0 - (player_pos.0.x + delta.x), player_pos.0.y + delta.y);
            for (tile, tile_pos) in map_query.iter() {
                let tile_pos = vec2(tile_pos.y as f32, tile_pos.x as f32);
                if tile_pos == target_tile {
                    if tile.is_floor {
                        let tiled_map = map.0.clone().unwrap();
                        let tiled_map = tiled_assets.get(&tiled_map.clone_weak()).unwrap();
                        player_pos.0 = vec2(player_pos.0.x + delta.x, player_pos.0.y + delta.y);
                        player.translation = from_tiled_position_to_world_space(
                            tiled_map,
                            &TilemapAnchor::TopLeft,
                            player_pos.0 * 32.0,
                        )
                        .extend(1.0)
                    }
                    break;
                }
            }
        }
    }
}
