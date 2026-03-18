use bevy::prelude::*;

use crate::components::common::Velocity;
use crate::components::player::*;
use crate::components::tail::*;
use crate::resources::game_config::GameConfig;
use crate::states::GameState;
use crate::systems::GameSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(
                Update,
                (
                    player_input.in_set(GameSystemSet::Input),
                    apply_thrust.in_set(GameSystemSet::Physics),
                    apply_velocity.in_set(GameSystemSet::Physics),
                    sync_player_rotation.in_set(GameSystemSet::Physics),
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Heading(0.0),
        Thrusting(false),
        Velocity::default(),
        PlayerStats::default(),
        TailChain::default(),
        PositionHistory::default(),
        Sprite {
            color: Color::srgb(0.2, 0.6, 1.0),
            custom_size: Some(Vec2::new(24.0, 14.0)),
            ..default()
        },
        Transform::from_translation(Vec3::ZERO),
    ));
}

fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Heading, &mut Thrusting), With<Player>>,
    config: Res<GameConfig>,
    time: Res<Time>,
) {
    let Ok((mut heading, mut thrusting)) = query.get_single_mut() else {
        return;
    };

    let mut rotation_delta = 0.0;
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        rotation_delta += config.rotation_speed * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        rotation_delta -= config.rotation_speed * time.delta_secs();
    }
    heading.0 += rotation_delta;

    thrusting.0 = keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp);
}

fn apply_thrust(
    mut query: Query<(&Heading, &Thrusting, &PlayerStats, &mut Velocity), With<Player>>,
    config: Res<GameConfig>,
    time: Res<Time>,
) {
    let Ok((heading, thrusting, stats, mut velocity)) = query.get_single_mut() else {
        return;
    };

    if thrusting.0 {
        let direction = Vec2::new(heading.0.cos(), heading.0.sin());
        let acceleration = direction * (config.thrust_force / stats.mass);
        velocity.0 += acceleration * time.delta_secs();
    }

    let speed = velocity.0.length();
    if speed > config.max_velocity {
        velocity.0 = velocity.0.normalize() * config.max_velocity;
    }
}

fn apply_velocity(
    mut query: Query<(&Velocity, &mut Transform)>,
    time: Res<Time>,
) {
    for (velocity, mut transform) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

fn sync_player_rotation(
    mut query: Query<(&Heading, &mut Transform), With<Player>>,
) {
    let Ok((heading, mut transform)) = query.get_single_mut() else {
        return;
    };
    transform.rotation = Quat::from_rotation_z(heading.0);
}
