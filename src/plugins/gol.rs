use bevy::prelude::*;
use std::collections::HashSet;

use crate::components::gol::*;
use crate::resources::game_config::GameConfig;
use crate::resources::gol_grid::LifeGrid;
use crate::resources::level_config::*;
use crate::states::GameState;
use crate::systems::GameSystemSet;

pub struct GolPlugin;

impl Plugin for GolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_gol)
            .add_systems(
                Update,
                (
                    tick_gol.in_set(GameSystemSet::GolTick),
                    sync_gol_entities.in_set(GameSystemSet::GolTick).after(tick_gol),
                    check_wave_triggers.in_set(GameSystemSet::GolTick).after(sync_gol_entities),
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn setup_gol(
    mut commands: Commands,
    existing_level: Option<Res<CurrentLevel>>,
) {
    let level_number = existing_level
        .map(|l| l.level_number)
        .unwrap_or(1);

    let (patterns, waves, tick_rate) = get_level(level_number);

    let mut grid = LifeGrid::new(tick_rate);
    for pattern in &patterns {
        grid.add_pattern(&pattern.cells, pattern.offset);
    }

    commands.insert_resource(grid);
    commands.insert_resource(CurrentLevel {
        level_number,
        waves_remaining: waves,
    });
}

fn tick_gol(mut grid: ResMut<LifeGrid>, time: Res<Time>) {
    grid.tick_timer.tick(time.delta());
    if grid.tick_timer.just_finished() {
        grid.tick();
    }
}

fn sync_gol_entities(
    mut commands: Commands,
    grid: Res<LifeGrid>,
    config: Res<GameConfig>,
    existing_cells: Query<(Entity, &CellPosition), With<LifeCell>>,
) {
    let alive = &grid.alive_cells;

    for (entity, pos) in &existing_cells {
        if !alive.contains(&(pos.x, pos.y)) {
            commands.entity(entity).despawn();
        }
    }

    let existing_positions: HashSet<(i32, i32)> = existing_cells
        .iter()
        .map(|(_, pos)| (pos.x, pos.y))
        .collect();

    for &(x, y) in alive {
        if !existing_positions.contains(&(x, y)) {
            let world_pos = Vec3::new(
                x as f32 * config.cell_size,
                y as f32 * config.cell_size,
                0.0,
            );
            commands.spawn((
                LifeCell,
                CellPosition { x, y },
                Sprite {
                    color: Color::srgb(0.0, 1.0, 0.0),
                    custom_size: Some(Vec2::splat(config.cell_size * 0.9)),
                    ..default()
                },
                Transform::from_translation(world_pos),
            ));
        }
    }
}

fn check_wave_triggers(
    mut grid: ResMut<LifeGrid>,
    mut level: ResMut<CurrentLevel>,
) {
    let cell_count = grid.alive_cells.len();

    level.waves_remaining.retain(|wave| {
        let should_spawn = match &wave.trigger {
            WaveTrigger::CellCountBelow(n) => cell_count < *n,
        };

        if should_spawn {
            for pattern in &wave.patterns {
                grid.add_pattern(&pattern.cells, pattern.offset);
            }
            false
        } else {
            true
        }
    });
}
