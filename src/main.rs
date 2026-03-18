use bevy::prelude::*;

mod components;
mod plugins;
mod resources;
mod states;
mod systems;

use resources::game_config::GameConfig;
use states::GameState;
use systems::GameSystemSet;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Life's End".into(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.05)))
        .insert_resource(GameConfig::default())
        .init_state::<GameState>()
        .configure_sets(
            Update,
            (
                GameSystemSet::Input,
                GameSystemSet::Physics,
                GameSystemSet::TailUpdate,
                GameSystemSet::GolTick,
                GameSystemSet::Combat,
                GameSystemSet::Collision,
                GameSystemSet::Cleanup,
            )
                .chain(),
        )
        .add_plugins((
            plugins::camera::CameraPlugin,
            plugins::player::PlayerPlugin,
            plugins::gol::GolPlugin,
            plugins::combat::CombatPlugin,
            plugins::collision::CollisionPlugin,
            plugins::tail::TailPlugin,
            plugins::level::LevelPlugin,
            plugins::ui::UiPlugin,
        ))
        .run();
}
