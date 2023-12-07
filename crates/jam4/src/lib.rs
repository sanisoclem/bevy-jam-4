use bevy::prelude::*;
use bevy_smud::SmudPlugin;

pub mod boid;
mod components;
pub mod level;
mod mods;
pub mod moveable;
mod player;
mod state;

use boid::{calculate_boid_direction, draw_boid_gizmos, update_boid_velocity, BoidConfig};
pub use components::*;
use level::{on_loading, check_if_level_complete, check_if_game_over, LevelManager, LevelRegistry, on_load_level_requested};
pub use mods::*;
use moveable::{move_moveables, MoveableBounds};
pub use player::*;
pub use state::*;

pub trait Jam4Extensions {
  fn add_jam_game(&mut self) -> &mut Self;
}

impl Jam4Extensions for App {
  fn add_jam_game(&mut self) -> &mut Self {
    self
      .add_plugins(SmudPlugin)
      .init_resource::<ModManager>()
      .init_resource::<MoveableBounds>()
      .init_resource::<BoidConfig>()
      .init_resource::<LevelRegistry>()
      .init_resource::<LevelManager>()
      .add_state::<SimulationState>()
      .add_event::<GameControlCommand>()
      .add_systems(OnExit(SimulationState::Disabled), register_mods)
      .add_systems(OnEnter(SimulationState::Ready), run_mod_init)
      .add_systems(
        OnEnter(SimulationState::Loading),
        (on_loading, on_load_level_requested, apply_deferred, run_mod_setup).chain(),
      )
      .add_systems(
        Update,
        (
          process_game_control_commands,
          (
            run_mod_update,
            draw_boid_gizmos,
            check_if_level_complete,
            check_if_game_over,
            (
              calculate_boid_direction,
              update_boid_velocity,
              move_moveables,
            )
              .chain(),
          )
            .run_if(in_state(SimulationState::Simulating)),
          wait_until_loading_complete.run_if(in_state(SimulationState::Loading)),
        ),
      )
  }
}
