use bevy::prelude::*;

use crate::components::gol::LifeCell;
use crate::components::tail::TailChain;
use crate::components::player::Player;
use crate::resources::level_config::CurrentLevel;
use crate::states::GameState;

pub struct UiPlugin;

#[derive(Component)]
struct MenuUi;

#[derive(Component)]
struct HudUi;

#[derive(Component)]
struct HudCellCount;

#[derive(Component)]
struct HudTailLength;

#[derive(Component)]
struct HudLevelNumber;

#[derive(Component)]
struct GameOverUi;

#[derive(Component)]
struct LevelCompleteUi;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(OnExit(GameState::Menu), despawn_tagged::<MenuUi>)
            .add_systems(Update, menu_input.run_if(in_state(GameState::Menu)))
            .add_systems(OnEnter(GameState::Playing), setup_hud)
            .add_systems(OnExit(GameState::Playing), despawn_tagged::<HudUi>)
            .add_systems(Update, (update_hud, pause_input).run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::Paused), setup_pause_overlay)
            .add_systems(OnExit(GameState::Paused), despawn_tagged::<GameOverUi>)
            .add_systems(Update, unpause_input.run_if(in_state(GameState::Paused)))
            .add_systems(OnEnter(GameState::GameOver), setup_game_over)
            .add_systems(OnExit(GameState::GameOver), despawn_tagged::<GameOverUi>)
            .add_systems(Update, game_over_input.run_if(in_state(GameState::GameOver)))
            .add_systems(OnEnter(GameState::LevelComplete), setup_level_complete)
            .add_systems(OnExit(GameState::LevelComplete), despawn_tagged::<LevelCompleteUi>)
            .add_systems(Update, level_complete_input.run_if(in_state(GameState::LevelComplete)));
    }
}

fn despawn_tagged<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_menu(mut commands: Commands) {
    commands
        .spawn((
            MenuUi,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("LIFE'S END"),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 0.0)),
            ));
            parent.spawn((
                Text::new("Eliminate all life."),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
            parent.spawn((
                Text::new("[SPACE] Start  |  WASD: Move  |  SPACE: Shoot"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));
        });
}

fn menu_input(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::Space) || keyboard.just_pressed(KeyCode::Enter) {
        next_state.set(GameState::Playing);
    }
}

fn setup_hud(mut commands: Commands, level: Option<Res<CurrentLevel>>) {
    let level_num = level.map(|l| l.level_number).unwrap_or(1);
    commands
        .spawn((
            HudUi,
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(10.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                HudTailLength,
                Text::new("Tail: 0"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.2, 0.6, 1.0)),
            ));
            parent.spawn((
                HudLevelNumber,
                Text::new(format!("Level {}", level_num)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.2)),
            ));
            parent.spawn((
                HudCellCount,
                Text::new("Cells: 0"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 0.0)),
            ));
        });
}

fn update_hud(
    cells: Query<&LifeCell>,
    player: Query<&TailChain, With<Player>>,
    mut cell_text: Query<&mut Text, With<HudCellCount>>,
    mut tail_text: Query<&mut Text, (With<HudTailLength>, Without<HudCellCount>)>,
) {
    let cell_count = cells.iter().count();
    if let Ok(mut text) = cell_text.get_single_mut() {
        **text = format!("Cells: {}", cell_count);
    }

    if let Ok(chain) = player.get_single() {
        if let Ok(mut text) = tail_text.get_single_mut() {
            **text = format!("Tail: {}", chain.segments.len());
        }
    }
}

fn pause_input(keyboard: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

fn setup_pause_overlay(mut commands: Commands) {
    commands
        .spawn((
            GameOverUi,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("PAUSED"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
            parent.spawn((
                Text::new("[ESC] Resume"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

fn unpause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Playing);
    }
}

fn setup_game_over(mut commands: Commands) {
    commands
        .spawn((
            GameOverUi,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.3, 0.0, 0.0, 0.7)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("LIFE CONSUMED YOU"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.2, 0.2)),
            ));
            parent.spawn((
                Text::new("[R] Retry  |  [ESC] Menu"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

fn game_over_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        next_state.set(GameState::Playing);
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}

fn setup_level_complete(mut commands: Commands) {
    commands
        .spawn((
            LevelCompleteUi,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.1, 0.0, 0.7)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("LIFE EXTINGUISHED"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 0.0)),
            ));
            parent.spawn((
                Text::new("[SPACE] Next Level  |  [ESC] Menu"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
            ));
        });
}

fn level_complete_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Playing);
    }
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}
