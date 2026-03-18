use bevy::prelude::*;

use crate::components::combat::*;
use crate::components::common::Velocity;
use crate::components::player::*;
use crate::resources::game_config::GameConfig;
use crate::states::GameState;
use crate::systems::GameSystemSet;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CellDestroyed>().add_systems(
            Update,
            (
                player_shoot.in_set(GameSystemSet::Combat),
                move_projectiles.in_set(GameSystemSet::Combat),
                expire_projectiles.in_set(GameSystemSet::Cleanup),
            )
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn player_shoot(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    player: Query<(&Transform, &Heading), With<Player>>,
    config: Res<GameConfig>,
) {
    if !keyboard.just_pressed(KeyCode::Space) && !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok((transform, heading)) = player.get_single() else {
        return;
    };

    let direction = Vec2::new(heading.0.cos(), heading.0.sin());
    let spawn_pos = transform.translation + (direction * 15.0).extend(0.0);

    commands.spawn((
        Projectile {
            lifetime: config.projectile_lifetime,
        },
        Velocity(direction * config.projectile_speed),
        Sprite {
            color: Color::srgb(1.0, 1.0, 0.4),
            custom_size: Some(Vec2::splat(6.0)),
            ..default()
        },
        Transform::from_translation(spawn_pos),
    ));
}

fn move_projectiles(
    mut query: Query<(&Velocity, &mut Transform), With<Projectile>>,
    time: Res<Time>,
) {
    for (velocity, mut transform) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

fn expire_projectiles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Projectile)>,
    time: Res<Time>,
) {
    for (entity, mut projectile) in &mut query {
        projectile.lifetime -= time.delta_secs();
        if projectile.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
