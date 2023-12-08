use crate::{
  moveable::{CollidedWithBounds, Moveable, MoveableBounds},
  Player, PlayerInfo,
};
use bevy::prelude::*;

mod components;
mod config;

pub use components::*;
pub use config::*;

pub fn despawn_collided_boids(
  mut cmd: Commands,
  qry: Query<Entity, (With<Boid>, Added<CollidedWithBounds>)>,
) {
  for e in qry.iter() {
    cmd.entity(e).despawn_recursive();
  }
}

pub fn calc_tamed_boids(
  mut cmd: Commands,
  mut qry: Query<(Entity, &Transform, &mut Handle<ColorMaterial>), (With<Boid>, Without<Player>)>,
  qry_player: Query<(&Transform, &Boid), With<Player>>,
  qry_check: Query<Entity, (With<Boid>, Without<Player>, With<TamedBoid>)>,
  bconfig: Res<BoidConfig>,
) {
  let Ok((p_trans, p_boid)) = qry_player.get_single() else {
    return;
  };
  for (e, transform, mut boid_color) in qry.iter_mut() {
    let prev_is_tamed = qry_check.get(e).is_ok();
    let is_tamed =
      transform.translation.distance_squared(p_trans.translation) <= p_boid.vision * p_boid.vision;
    if is_tamed && !prev_is_tamed {
      cmd.entity(e).insert(TamedBoid);
      *boid_color = bconfig.color_tamed.clone();
    }
    if !is_tamed && prev_is_tamed {
      cmd.entity(e).remove::<TamedBoid>();
      *boid_color = bconfig.color_wild.clone();
    }
  }
}

pub fn draw_boid_gizmos(
  qry: Query<(&Transform, &Boid)>,
  mut gizmos: Gizmos,
  bconfig: Res<BoidConfig>,
) {
  for (t, b) in qry.iter() {
    let pos = t.translation.xy();
    if bconfig.show_vision {
      gizmos.circle_2d(pos, b.vision, Color::PURPLE);
    }
    if bconfig.show_personal_space {
      gizmos.circle_2d(pos, b.personal_space, Color::RED);
    }
    if bconfig.show_direction {
      gizmos.ray_2d(pos, b.direction.xy() * b.vision, Color::GREEN);
    }
  }
}

pub fn calculate_boid_direction(
  mut qry: Query<(Entity, &Transform, &mut Boid, Option<&TamedBoid>)>,
  mut gizmos: Gizmos,
  bounds: Res<MoveableBounds>,
  bconfig: Res<BoidConfig>,
  time: Res<Time>,
) {
  let changes = qry
    .iter()
    .map(|(e, t1, boid, tamed)| {
      let pos = t1.translation.xy();
      let (force, speed_change) = boid.calculate_forces(&qry, &bconfig, pos, &bounds, &mut gizmos);

      if bconfig.show_forces {
        gizmos.ray_2d(pos, force * boid.vision, Color::CYAN);
      }

      (e, force, pos, speed_change, tamed.is_some())
    })
    .collect::<Vec<_>>();

  for (e, f, pos, speed_change, is_tamed) in changes.iter() {
    let (_, _, mut b, _) = qry.get_mut(*e).unwrap();

    b.direction = (b.direction + (*f * time.delta_seconds() * b.turning_speed)).normalize();

    if *is_tamed {
      b.speed = b.speed + (speed_change - b.speed);
    } else if !b.is_player {
      b.speed = bconfig.min_speed;
    }

    if bconfig.show_direction {
      gizmos.ray_2d(*pos, b.direction * 100.0, Color::BLUE);
    }
  }
}

pub fn update_tamed_boids(
  mut qry: Query<&mut Boid>,
  added: Query<Entity, Added<TamedBoid>>,
  mut removed: RemovedComponents<TamedBoid>,
  player: Res<PlayerInfo>,
  bconfig: Res<BoidConfig>,
) {
  for (entity) in added.iter() {
    let Ok(mut boid) = qry.get_mut(entity) else {
      continue;
    };

    if player.in_boost_mode {
      boid.speed = bconfig.max_speed;
      boid.turning_speed = bconfig.min_turn_speed;
    } else {
      boid.speed = bconfig.min_speed;
      boid.turning_speed = bconfig.max_turn_speed;
    }
  }

  for entity in removed.read() {
    let Ok(mut boid) = qry.get_mut(entity) else {
      continue;
    };
    boid.speed = bconfig.min_speed;
    boid.turning_speed = bconfig.max_turn_speed;
  }
}

pub fn update_boid_velocity(mut qry: Query<(&mut Moveable, &mut Transform, &Boid)>) {
  for (mut mov, mut t, boid) in qry.iter_mut() {
    let normalized = if boid.direction == Vec2::ZERO {
      Vec3::Y
    } else {
      boid.direction.extend(0.0).normalize()
    };

    t.rotation =
      Quat::from_rotation_z(normalized.x.signum() * -1. * normalized.angle_between(Vec3::Y));

    mov.velocity = normalized * boid.speed;
  }
}
