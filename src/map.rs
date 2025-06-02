use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{map_string, state::GameState};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .insert_resource(Map::new(
                7,
                7,
                vec2(1.0, 1.0),
                map_string!(
                    "0000000", // 0
                    "0222220", // 1
                    "0222220", // 2
                    "0222220", // 3
                    "0222220", // 4
                    "0222220", // 5
                    "0000000"  // 6
                ),
            ))
            .add_systems(Startup, setup_map)
            .add_systems(Update, update_map.run_if(run_once));
    }
}

#[derive(Resource, Clone)]
pub struct Map {
    pub map: String,
    pub width: u32,
    pub height: u32,
    pub player_pos: Vec2,
}

impl Map {
    pub fn new(width: u32, height: u32, player_pos: Vec2, map: String) -> Self {
        Map {
            width,
            height,
            map,
            player_pos,
        }
    }
    pub fn get(&self, x: i32, y: i32) -> u32 {
        self.map
            .chars()
            .nth((x * (self.width as i32) + y) as usize)
            .unwrap()
            .to_digit(10)
            .unwrap()
    }

    pub fn move_player(&mut self, delta_x: i32, delta_y: i32) -> bool {
        if delta_x != 0
            && !self.is_wall(
                (self.player_pos.x + delta_x as f32) as i32,
                self.player_pos.y as i32,
            )
        {
            self.player_pos.x += delta_x as f32;
            return true;
        } else if delta_y != 0
            && !self.is_wall(
                self.player_pos.x as i32,
                (self.player_pos.y + delta_y as f32) as i32,
            )
        {
            self.player_pos.y += delta_y as f32;
            return true;
        }
        false
    }

    fn is_wall(&self, x: i32, y: i32) -> bool {
        self.get(x, y) == 0
    }
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>, map: Res<Map>) {
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size = TilemapSize {
        x: map.width,
        y: map.height,
    };
    let mut tile_storage = TileStorage::empty(map_size);
    let map_type = TilemapType::Square;

    let tilemap_entity = commands.spawn_empty().id();

    for y in 0..map_size.y {
        for x in 0..map_size.x {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();

    // Spawns a tilemap.
    commands.entity(tilemap_entity).insert((TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        map_type,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        anchor: TilemapAnchor::TopLeft,
        ..Default::default()
    },));
}

fn update_map(
    mut state: ResMut<State<GameState>>,
    mut tilemap_query: Query<(&TileStorage, &TilemapSize)>,
    mut tile_query: Query<&mut TileTextureIndex>,
    map: Res<Map>,
) {
    if state.get() == &GameState::LoadMap {
        for (tile_storage, _tile_size) in tilemap_query.iter_mut() {
            for x in 0..map.width {
                for y in 0..map.height {
                    if let Some(tile) = tile_storage.get(&TilePos { x, y }) {
                        if let Ok(mut tile_texture) = tile_query.get_mut(tile) {
                            tile_texture.0 = map.get(x as i32, y as i32);
                        }
                    }
                }
            }
        }
        *state = State::new(GameState::LoadPlayer);
    }
}
