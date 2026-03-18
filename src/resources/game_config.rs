use bevy::prelude::*;

#[derive(Resource)]
pub struct GameConfig {
    pub cell_size: f32,
    pub thrust_force: f32,
    pub brake_force: f32,
    pub rotation_speed: f32,
    pub max_velocity: f32,
    pub friction: f32,
    pub projectile_speed: f32,
    pub projectile_lifetime: f32,
    pub player_base_mass: f32,
    pub mass_per_segment: f32,
    pub arena_half_width: f32,
    pub arena_half_height: f32,
    pub bounce_damping: f32,
    pub safe_spawn_radius: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            cell_size: 16.0,
            thrust_force: 300.0,
            brake_force: 150.0,
            rotation_speed: 7.0,
            max_velocity: 350.0,
            friction: 0.98,
            projectile_speed: 600.0,
            projectile_lifetime: 1.5,
            player_base_mass: 1.0,
            mass_per_segment: 0.12,
            arena_half_width: 500.0,
            arena_half_height: 350.0,
            bounce_damping: 0.7,
            safe_spawn_radius: 5.0,
        }
    }
}
