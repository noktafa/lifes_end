use bevy::prelude::*;

use crate::components::combat::*;
use crate::components::common::Velocity;
use crate::components::gol::LifeCell;
use crate::components::player::*;
use crate::resources::game_config::GameConfig;
use crate::states::GameState;
use crate::systems::GameSystemSet;

const AIM_ASSIST_STRENGTH: f32 = 0.10;
const AIM_ASSIST_CONE: f32 = 0.5; // ~28 degrees half-angle (radians)
const AIM_ASSIST_RANGE: f32 = 400.0;

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
    cells: Query<&Transform, With<LifeCell>>,
    config: Res<GameConfig>,
) {
    if !keyboard.just_pressed(KeyCode::Space) && !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok((transform, heading)) = player.get_single() else {
        return;
    };

    let aim_dir = Vec2::new(heading.0.cos(), heading.0.sin());
    let player_pos = transform.translation.truncate();

    // Find nearest cell within aim cone for aim assist
    let mut best_target: Option<Vec2> = None;
    let mut best_dist = f32::MAX;

    for cell_transform in &cells {
        let cell_pos = cell_transform.translation.truncate();
        let to_cell = cell_pos - player_pos;
        let dist = to_cell.length();

        if dist > AIM_ASSIST_RANGE || dist < 1.0 {
            continue;
        }

        let to_cell_norm = to_cell.normalize();
        let angle = aim_dir.dot(to_cell_norm).acos();

        if angle < AIM_ASSIST_CONE && dist < best_dist {
            best_dist = dist;
            best_target = Some(to_cell_norm);
        }
    }

    // Blend aim direction toward target by 10%
    let final_dir = if let Some(target_dir) = best_target {
        let blended = aim_dir * (1.0 - AIM_ASSIST_STRENGTH) + target_dir * AIM_ASSIST_STRENGTH;
        blended.normalize()
    } else {
        aim_dir
    };

    let spawn_pos = transform.translation + (final_dir * 15.0).extend(0.0);

    commands.spawn((
        Projectile {
            lifetime: config.projectile_lifetime,
        },
        Velocity(final_dir * config.projectile_speed),
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
