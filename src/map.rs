use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::{map_string, state::GameState};

const TILEMAP_SIZE: u32 = 5;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .add_systems(Startup, setup_map)
            .add_systems(Update, update_map.run_if(run_once));
    }
}

#[derive(Component)]
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
    pub fn get(&self, x: u32, y: u32) -> u32 {
        self.map
            .chars()
            .nth((x * self.width + y) as usize)
            .unwrap()
            .to_digit(10)
            .unwrap()
    }
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Map::new(
        5,
        5,
        vec2(1.0, 1.0),
        map_string!(
            "00000", // 0
            "01110", // 1
            "01110", // 2
            "01110", // 3
            "00000"  // 4
        ),
    ));

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size = TilemapSize {
        x: TILEMAP_SIZE,
        y: TILEMAP_SIZE,
    };
    let mut tile_storage = TileStorage::empty(map_size);
    let map_type = TilemapType::Square;

    let tilemap_entity = commands.spawn_empty().id();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
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
    map: Query<&Map>,
) {
    if state.get() == &GameState::LoadMap {
        let current_map = map.single().unwrap();
        for (tile_storage, _tile_size) in tilemap_query.iter_mut() {
            for x in 0..TILEMAP_SIZE {
                for y in 0..TILEMAP_SIZE {
                    if let Some(tile) = tile_storage.get(&TilePos { x, y }) {
                        if let Ok(mut tile_texture) = tile_query.get_mut(tile) {
                            tile_texture.0 = current_map.get(x, y);
                        }
                    }
                }
            }
        }
        *state = State::new(GameState::LoadPlayer);
    }
}
