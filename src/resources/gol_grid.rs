use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Resource)]
pub struct LifeGrid {
    pub alive_cells: HashSet<(i32, i32)>,
    pub tick_timer: Timer,
}

impl LifeGrid {
    pub fn new(tick_rate: f32) -> Self {
        Self {
            alive_cells: HashSet::new(),
            tick_timer: Timer::from_seconds(tick_rate, TimerMode::Repeating),
        }
    }

    pub fn tick(&mut self) {
        let mut neighbor_counts: HashMap<(i32, i32), u8> = HashMap::new();

        for &(x, y) in &self.alive_cells {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    *neighbor_counts.entry((x + dx, y + dy)).or_insert(0) += 1;
                }
            }
        }

        let mut next_gen = HashSet::new();
        for (&pos, &count) in &neighbor_counts {
            if count == 3 || (count == 2 && self.alive_cells.contains(&pos)) {
                next_gen.insert(pos);
            }
        }

        self.alive_cells = next_gen;
    }

    pub fn add_pattern(&mut self, cells: &[(i32, i32)], offset: (i32, i32)) {
        for &(x, y) in cells {
            self.alive_cells.insert((x + offset.0, y + offset.1));
        }
    }
}
