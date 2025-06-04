use std::env;

use bevy::prelude::*;
use bevy_ecs_tiled::{
    TiledMapPlugin, TiledMapPluginConfig, map::TiledMapHandle, prelude::TiledMap,
};
use bevy_ecs_tilemap::prelude::*;

use crate::state::GameState;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let mut path = env::current_dir().unwrap();
        path.push("output");
        path.push("tiled_export_file.json");
        app.add_plugins(TilemapPlugin)
            .add_plugins(TiledMapPlugin(TiledMapPluginConfig {
                tiled_types_export_file: Some(path),
            }))
            .register_type::<MapTile>()
            .register_type::<PlayerSpawn>()
            .insert_resource(CurrentMap(None))
            .add_systems(Startup, setup_map)
            .add_systems(Update, update_state.run_if(run_once));
    }
}

#[derive(Default, Debug, Component, Reflect)]
#[reflect(Default, Component)]
pub struct MapTile {
    pub is_wall: bool,
    pub is_floor: bool,
}

#[derive(Default, Debug, Component, Reflect)]
#[reflect(Default, Component)]
pub struct PlayerSpawn {
    pub position: Vec2,
}

#[derive(Resource)]
pub struct CurrentMap(pub Option<Handle<TiledMap>>);

fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut current_map: ResMut<CurrentMap>,
) {
    let map: Handle<TiledMap> = asset_server.load("maps/level.tmx");
    current_map.0 = Some(map.clone());
    commands.spawn((
        TiledMapHandle(map),
        TilemapAnchor::TopLeft,
        Transform::from_xyz(0.0, 0.0, 0.1),
    ));
}

fn update_state(mut state: ResMut<State<GameState>>) {
    *state = State::new(GameState::LoadPlayer);
}
