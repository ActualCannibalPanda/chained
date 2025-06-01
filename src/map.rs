use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_map)
            .add_systems(Update, update_map.run_if(run_once));
    }
}

fn setup_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    let map_size = TilemapSize { x: 2, y: 2 };
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
        anchor: TilemapAnchor::Center,
        ..Default::default()
    },));
}

fn update_map(mut tile_query: Query<&mut TileTextureIndex>) {
    for mut tile in tile_query.iter_mut() {
        tile.0 = 1;
    }
}
