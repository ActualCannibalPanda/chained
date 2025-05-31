use bevy::prelude::*;

const TILE_SIZE: f32 = 32.0;

#[derive(Component)]
struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, update_player);
    }
}

fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        delta.y -= TILE_SIZE;
    }
    player.translation += delta;
}
