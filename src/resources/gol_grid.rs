use bevy::prelude::*;
use rand::Rng;
use std::collections::{HashMap, HashSet};

#[derive(Resource)]
pub struct LifeGrid {
    pub alive_cells: HashSet<(i32, i32)>,
    pub tick_timer: Timer,
    pub bounds: (i32, i32), // half-width, half-height in grid coords
    cell_age: HashMap<(i32, i32), u16>,
}

impl LifeGrid {
    pub fn new(tick_rate: f32, bounds: (i32, i32)) -> Self {
        Self {
            alive_cells: HashSet::new(),
            tick_timer: Timer::from_seconds(tick_rate, TimerMode::Repeating),
            bounds,
            cell_age: HashMap::new(),
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

        let (bw, bh) = self.bounds;
        let mut next_gen = HashSet::new();
        for (&pos, &count) in &neighbor_counts {
            if pos.0.abs() > bw || pos.1.abs() > bh {
                continue;
            }
            if count == 3 || (count == 2 && self.alive_cells.contains(&pos)) {
                next_gen.insert(pos);
            }
        }

        // Update cell ages: survivors get +1, new cells start at 1, dead cells removed
        let mut new_ages: HashMap<(i32, i32), u16> = HashMap::new();
        for &pos in &next_gen {
            let age = self.cell_age.get(&pos).copied().unwrap_or(0) + 1;
            new_ages.insert(pos, age);
        }
        self.cell_age = new_ages;

        // Stagnation mutation: cells alive for 16+ ticks spawn a random neighbor
        let mut rng = rand::thread_rng();
        let stale: Vec<(i32, i32)> = self
            .cell_age
            .iter()
            .filter(|&(_, &age)| age >= 16)
            .map(|(&pos, _)| pos)
            .collect();

        for pos in stale {
            // Find empty adjacent positions
            let mut candidates: Vec<(i32, i32)> = Vec::new();
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let n = (pos.0 + dx, pos.1 + dy);
                    if n.0.abs() <= bw && n.1.abs() <= bh && !next_gen.contains(&n) {
                        candidates.push(n);
                    }
                }
            }
            if !candidates.is_empty() {
                let pick = candidates[rng.gen_range(0..candidates.len())];
                next_gen.insert(pick);
                self.cell_age.insert(pick, 0);
            }
            // Reset the stale cell's age so it doesn't mutate every tick
            self.cell_age.insert(pos, 0);
        }

        self.alive_cells = next_gen;
    }

    pub fn add_pattern(&mut self, cells: &[(i32, i32)], offset: (i32, i32)) {
        for &(x, y) in cells {
            self.alive_cells.insert((x + offset.0, y + offset.1));
        }
    }

    pub fn clear_radius(&mut self, center: (i32, i32), radius: i32) {
        self.alive_cells.retain(|&(x, y)| {
            let dx = x - center.0;
            let dy = y - center.1;
            dx * dx + dy * dy > radius * radius
        });
    }
}
