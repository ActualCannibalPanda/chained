use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowResolution;

const TILE_SIZE: f32 = 32.0;

#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .add_plugins(
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
        )
        .add_systems(Startup, setup)
        .add_systems(Update, update_player)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            image: asset_server.load("player.png"),
            image_mode: SpriteImageMode::Scale(ScalingMode::FitCenter),
            ..default()
        },
        Player {},
    ));
}

fn update_player(input: Res<ButtonInput<KeyCode>>, mut query: Query<(&Player, &mut Transform)>) {
    let mut player = query.single_mut().unwrap().1;
    let mut delta = Vec3::splat(0.0);
    if input.just_pressed(KeyCode::KeyA) {
        delta.x -= TILE_SIZE;
    }
    if input.just_pressed(KeyCode::KeyD) {
        delta.x += TILE_SIZE;
    }
    if input.just_pressed(KeyCode::KeyW) {
        delta.y += TILE_SIZE;
    }
    if input.just_pressed(KeyCode::KeyS) {
        delta.y -= TILE_SIZE
    }
    player.translation += delta;
}
