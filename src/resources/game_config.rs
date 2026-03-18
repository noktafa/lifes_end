use bevy::prelude::*;

#[derive(Resource)]
pub struct GameConfig {
    pub cell_size: f32,
    pub thrust_force: f32,
    pub rotation_speed: f32,
    pub max_velocity: f32,
    pub projectile_speed: f32,
    pub projectile_lifetime: f32,
    pub player_base_mass: f32,
    pub mass_per_segment: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            cell_size: 16.0,
            thrust_force: 200.0,
            rotation_speed: 5.0,
            max_velocity: 400.0,
            projectile_speed: 600.0,
            projectile_lifetime: 2.0,
            player_base_mass: 1.0,
            mass_per_segment: 0.15,
        }
    }
}
