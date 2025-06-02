use bevy::{prelude::*, sprite::Anchor};

const CHAIN_BUFFER: i32 = 5;

#[derive(Component)]
pub struct ChainPos(pub Vec2);

impl Default for ChainPos {
    fn default() -> Self {
        Self(Vec2::new(-1000.0, -1000.0))
    }
}

pub struct ChainPlugin;

impl Plugin for ChainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_chain);
    }
}

fn setup_chain(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..CHAIN_BUFFER {
        commands.spawn((
            Sprite {
                image: asset_server.load::<Image>("chain.png"),
                anchor: Anchor::TopLeft,
                ..default()
            },
            Transform::from_xyz(-100.0 + (i as f32) * 32.0, -100.0, 0.9),
            ChainPos(vec2(0.0, 0.0)),
        ));
    }
}
