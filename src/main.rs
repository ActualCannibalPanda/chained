use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use chained::chain::ChainPlugin;
use chained::cursor::CursorPlugin;
use chained::map::MapPlugin;
use chained::player::PlayerPlugin;
use chained::state::GameState;

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
                        resolution: WindowResolution::new(800.0, 600.0),
                        ..default()
                    }),
                    ..default()
                }),
            ChainPlugin,
            CursorPlugin,
            MapPlugin,
            PlayerPlugin,
        ))
        .insert_state(GameState::LoadMap)
        .insert_resource(ClearColor(Color::srgb(0.41, 0.42, 0.42)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(400.0, -300.0, 1.0)));
}
