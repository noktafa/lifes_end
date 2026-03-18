use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub lifetime: f32,
}

#[derive(Event)]
pub struct CellDestroyed;
