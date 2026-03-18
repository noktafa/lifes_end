use bevy::prelude::*;

use crate::components::player::Player;
use crate::resources::game_config::GameConfig;
use crate::states::GameState;

pub struct CameraPlugin;

#[derive(Component)]
pub struct GameCamera;

#[derive(Component)]
pub struct ArenaBorder;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(OnEnter(GameState::Playing), spawn_arena_borders)
            .add_systems(OnExit(GameState::Playing), despawn_arena_borders)
            .add_systems(Update, camera_follow_player.run_if(in_state(GameState::Playing)));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, GameCamera));
}

fn spawn_arena_borders(mut commands: Commands, config: Res<GameConfig>) {
    let hw = config.arena_half_width;
    let hh = config.arena_half_height;
    let thickness = 4.0;
    let border_color = Color::srgb(0.15, 0.15, 0.25);

    // Top
    commands.spawn((
        ArenaBorder,
        Sprite {
            color: border_color,
            custom_size: Some(Vec2::new(hw * 2.0 + thickness * 2.0, thickness)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, hh + thickness / 2.0, -1.0)),
    ));
    // Bottom
    commands.spawn((
        ArenaBorder,
        Sprite {
            color: border_color,
            custom_size: Some(Vec2::new(hw * 2.0 + thickness * 2.0, thickness)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, -hh - thickness / 2.0, -1.0)),
    ));
    // Left
    commands.spawn((
        ArenaBorder,
        Sprite {
            color: border_color,
            custom_size: Some(Vec2::new(thickness, hh * 2.0 + thickness * 2.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(-hw - thickness / 2.0, 0.0, -1.0)),
    ));
    // Right
    commands.spawn((
        ArenaBorder,
        Sprite {
            color: border_color,
            custom_size: Some(Vec2::new(thickness, hh * 2.0 + thickness * 2.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(hw + thickness / 2.0, 0.0, -1.0)),
    ));
}

fn despawn_arena_borders(mut commands: Commands, borders: Query<Entity, With<ArenaBorder>>) {
    for entity in &borders {
        commands.entity(entity).despawn();
    }
}

fn camera_follow_player(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
) {
    let Ok(player_transform) = player.get_single() else {
        return;
    };
    let Ok(mut camera_transform) = camera.get_single_mut() else {
        return;
    };
    let target = player_transform.translation;
    camera_transform.translation = camera_transform.translation.lerp(target, 0.08);
    camera_transform.translation.z = 999.0;
}
