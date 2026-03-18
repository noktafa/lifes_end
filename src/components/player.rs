use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct Heading(pub f32);

#[derive(Component, Default)]
pub struct Thrusting(pub bool);

#[derive(Component)]
pub struct PlayerStats {
    pub mass: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self { mass: 1.0 }
    }
}
