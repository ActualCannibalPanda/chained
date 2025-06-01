use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_ecs_tilemap::prelude::*;
use map::MapPlugin;
use player::PlayerPlugin;

mod map;
mod player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Chained"),
                        name: Some(String::from("Chained")),
                        resolution: WindowResolution::new(800.0, 600.0)
                            .with_scale_factor_override(1.5),
                        ..default()
                    }),
                    ..default()
                }),
            PlayerPlugin,
            MapPlugin,
            TilemapPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
