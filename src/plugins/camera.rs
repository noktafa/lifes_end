use bevy::prelude::*;

use crate::components::player::Player;
use crate::states::GameState;

pub struct CameraPlugin;

#[derive(Component)]
pub struct GameCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, camera_follow_player.run_if(in_state(GameState::Playing)));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, GameCamera));
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
    camera_transform.translation = camera_transform.translation.lerp(target, 0.05);
    camera_transform.translation.z = 999.0;
}
