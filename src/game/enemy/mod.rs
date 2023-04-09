use crate::states::AppState;
use bevy::prelude::*;

pub mod components;
mod events;
mod systems;

use events::*;
use systems::*;

use super::states::SimulationState;

const ENEMY_SPEED: f32 = 50.;
const ENEMY_SIZE: f32 = 16.;
const NUMBER_OF_ENEMIES: usize = 1;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyHitWallEvent>()
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            .add_system(
                move_enemy_controller
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(
                play_enemy_wall_hit_sound
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(
                check_for_world_collisions
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}
