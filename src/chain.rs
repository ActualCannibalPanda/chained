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
        app.add_systems(Startup, setup_chain)
            .add_systems(Update, update_chain);
    }
}

fn setup_chain(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..CHAIN_BUFFER {
        commands.spawn((
            Sprite {
                image: asset_server.load("sprites/chain.png"),
                anchor: Anchor::TopLeft,
                ..default()
            },
            Visibility::Hidden,
            Transform::from_xyz(-100.0 + (i as f32) * 32.0, -100.0, 0.9),
            ChainPos(vec2(0.0, 0.0)),
        ));
    }
}

fn update_chain(mut chain_query: Query<(&mut Transform, &ChainPos)>) {
    for (mut transform, chain_pos) in chain_query.iter_mut() {
        transform.translation = vec3(chain_pos.0.x * 32.0, chain_pos.0.y * -32.0, 0.5);
    }
}
