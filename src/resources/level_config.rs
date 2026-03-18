use bevy::prelude::*;

#[derive(Resource)]
pub struct CurrentLevel {
    pub level_number: usize,
    pub waves_remaining: Vec<WaveConfig>,
}

pub struct WaveConfig {
    pub trigger: WaveTrigger,
    pub patterns: Vec<PatternPlacement>,
}

pub struct PatternPlacement {
    pub cells: Vec<(i32, i32)>,
    pub offset: (i32, i32),
}

pub enum WaveTrigger {
    CellCountBelow(usize),
}

// Classic Game of Life patterns
pub fn glider() -> Vec<(i32, i32)> {
    vec![(0, 0), (1, 0), (2, 0), (2, 1), (1, 2)]
}

pub fn block() -> Vec<(i32, i32)> {
    vec![(0, 0), (1, 0), (0, 1), (1, 1)]
}

pub fn blinker() -> Vec<(i32, i32)> {
    vec![(0, 0), (1, 0), (2, 0)]
}

pub fn toad() -> Vec<(i32, i32)> {
    vec![(1, 0), (2, 0), (3, 0), (0, 1), (1, 1), (2, 1)]
}

pub fn beacon() -> Vec<(i32, i32)> {
    vec![(0, 0), (1, 0), (0, 1), (3, 2), (2, 3), (3, 3)]
}

pub fn r_pentomino() -> Vec<(i32, i32)> {
    vec![(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)]
}

pub fn lwss() -> Vec<(i32, i32)> {
    // Lightweight spaceship
    vec![
        (0, 0), (3, 0),
        (4, 1),
        (0, 2), (4, 2),
        (1, 3), (2, 3), (3, 3), (4, 3),
    ]
}

pub fn pulsar() -> Vec<(i32, i32)> {
    vec![
        (2, 0), (3, 0), (4, 0), (8, 0), (9, 0), (10, 0),
        (0, 2), (5, 2), (7, 2), (12, 2),
        (0, 3), (5, 3), (7, 3), (12, 3),
        (0, 4), (5, 4), (7, 4), (12, 4),
        (2, 5), (3, 5), (4, 5), (8, 5), (9, 5), (10, 5),
        (2, 7), (3, 7), (4, 7), (8, 7), (9, 7), (10, 7),
        (0, 8), (5, 8), (7, 8), (12, 8),
        (0, 9), (5, 9), (7, 9), (12, 9),
        (0, 10), (5, 10), (7, 10), (12, 10),
        (2, 12), (3, 12), (4, 12), (8, 12), (9, 12), (10, 12),
    ]
}

pub fn get_level(level_number: usize) -> (Vec<PatternPlacement>, Vec<WaveConfig>, f32) {
    // Returns: (initial_patterns, waves, gol_tick_rate)
    match level_number {
        1 => (
            // Tutorial: just some still lifes and a blinker
            vec![
                PatternPlacement { cells: block(), offset: (5, 5) },
                PatternPlacement { cells: block(), offset: (-5, -5) },
                PatternPlacement { cells: blinker(), offset: (10, 0) },
            ],
            vec![],
            0.4,
        ),
        2 => (
            // Oscillators
            vec![
                PatternPlacement { cells: blinker(), offset: (8, 0) },
                PatternPlacement { cells: toad(), offset: (-8, 5) },
                PatternPlacement { cells: beacon(), offset: (0, -8) },
                PatternPlacement { cells: block(), offset: (12, 12) },
            ],
            vec![],
            0.35,
        ),
        3 => (
            // First glider
            vec![
                PatternPlacement { cells: glider(), offset: (-12, -8) },
                PatternPlacement { cells: block(), offset: (8, 8) },
                PatternPlacement { cells: blinker(), offset: (-6, 10) },
            ],
            vec![
                WaveConfig {
                    trigger: WaveTrigger::CellCountBelow(3),
                    patterns: vec![
                        PatternPlacement { cells: glider(), offset: (15, -10) },
                    ],
                },
            ],
            0.3,
        ),
        4 => (
            // Multiple gliders
            vec![
                PatternPlacement { cells: glider(), offset: (-15, -10) },
                PatternPlacement { cells: glider(), offset: (10, 12) },
                PatternPlacement { cells: toad(), offset: (0, 0) },
                PatternPlacement { cells: beacon(), offset: (-10, 8) },
            ],
            vec![
                WaveConfig {
                    trigger: WaveTrigger::CellCountBelow(5),
                    patterns: vec![
                        PatternPlacement { cells: glider(), offset: (20, 0) },
                        PatternPlacement { cells: glider(), offset: (-20, 0) },
                    ],
                },
            ],
            0.25,
        ),
        5 => (
            // R-pentomino (chaotic)
            vec![
                PatternPlacement { cells: r_pentomino(), offset: (0, 0) },
            ],
            vec![],
            0.2,
        ),
        6 => (
            // Spaceships
            vec![
                PatternPlacement { cells: lwss(), offset: (-15, 5) },
                PatternPlacement { cells: glider(), offset: (12, -8) },
                PatternPlacement { cells: glider(), offset: (-8, 12) },
                PatternPlacement { cells: block(), offset: (0, 0) },
            ],
            vec![
                WaveConfig {
                    trigger: WaveTrigger::CellCountBelow(8),
                    patterns: vec![
                        PatternPlacement { cells: lwss(), offset: (20, -5) },
                    ],
                },
            ],
            0.2,
        ),
        7 => (
            // Pulsar - complex oscillator
            vec![
                PatternPlacement { cells: pulsar(), offset: (-6, -6) },
                PatternPlacement { cells: glider(), offset: (15, 15) },
            ],
            vec![
                WaveConfig {
                    trigger: WaveTrigger::CellCountBelow(10),
                    patterns: vec![
                        PatternPlacement { cells: r_pentomino(), offset: (20, -15) },
                    ],
                },
            ],
            0.15,
        ),
        8 => (
            // Chaos level - multiple R-pentominoes
            vec![
                PatternPlacement { cells: r_pentomino(), offset: (-10, -10) },
                PatternPlacement { cells: r_pentomino(), offset: (10, 10) },
                PatternPlacement { cells: lwss(), offset: (0, -15) },
            ],
            vec![
                WaveConfig {
                    trigger: WaveTrigger::CellCountBelow(15),
                    patterns: vec![
                        PatternPlacement { cells: r_pentomino(), offset: (-20, 15) },
                        PatternPlacement { cells: glider(), offset: (25, -5) },
                        PatternPlacement { cells: glider(), offset: (-25, 5) },
                    ],
                },
            ],
            0.12,
        ),
        // Beyond level 8: loop with increased difficulty
        _ => {
            let n = level_number;
            let mut patterns = vec![
                PatternPlacement { cells: r_pentomino(), offset: (0, 0) },
                PatternPlacement { cells: pulsar(), offset: (-(n as i32) * 3, n as i32 * 3) },
            ];
            for i in 0..((n - 8).min(5)) {
                let angle = (i as f32) * std::f32::consts::TAU / 5.0;
                let dist = 15 + n as i32;
                let ox = (angle.cos() * dist as f32) as i32;
                let oy = (angle.sin() * dist as f32) as i32;
                patterns.push(PatternPlacement { cells: glider(), offset: (ox, oy) });
            }
            (
                patterns,
                vec![WaveConfig {
                    trigger: WaveTrigger::CellCountBelow(10 + n),
                    patterns: vec![
                        PatternPlacement { cells: r_pentomino(), offset: (25, 0) },
                        PatternPlacement { cells: lwss(), offset: (-25, 10) },
                    ],
                }],
                (0.1_f32).max(0.4 - n as f32 * 0.03),
            )
        }
    }
}
