use std::time::Duration;

use bevy::prelude::*;
use bevy_smud::prelude::*;
use jam4::{level::LevelInfo, moveable::MoveableBounds};
use sdfu::SDF;

pub fn build_level2(asset_server: &AssetServer) -> LevelInfo {
  let w = 2000.;
  let h = 10000.;
  let fbounds = Vec4::new(0.0, 10_000., 5_000., 5_000.);
  let a = 3.0 * w / 2.0;
  let h2 = w / (2.0 * ((h / f32::sqrt((a * a) + (h * h))).acos()).tan());

  let outer = sdfu::Box::new(Vec2::new(w + 3000., h + 3000.));
  let inner = sdfu::Box::new(Vec2::new(w, h));
  let t1 = sdfu::Triangle::new([Vec2::new(-w / 2., 0.), Vec2::new(w, h), Vec2::new(w, -h)]);
  let t2 = sdfu::Triangle::new([Vec2::new(-w, 0.), Vec2::new(-w, -h), Vec2::new(w / 2., -h)]);
  let t3 = sdfu::Triangle::new([Vec2::new(-w, 0.), Vec2::new(-w, h), Vec2::new(w / 2., h)]);
  let t4 = sdfu::Box::new(Vec2::new(w / 4.0, h2/2.0)).translate(Vec2::new(-w / 2.0, 0.0));
  let shape = outer
    .subtract(inner)
    .union(t1.union(t2).union(t3)).subtract(t4);

  let finish_bounds = sdfu::Box::new(Vec2::new(fbounds.z, fbounds.w)).translate(fbounds.xy());
  let terrain_shader = asset_server.load("preload/terrain2.wgsl");

  let s = SmudShape {
    color: Color::BLACK,
    sdf: terrain_shader,
    frame: Frame::Quad(50000.),
    ..default()
  };

  let lvl = LevelInfo {
    bounds: MoveableBounds::from_sdf(shape),
    finish_bounds_box: MoveableBounds::from_sdf(finish_bounds),
    finish_bounds: fbounds,
    bounds_sdf: Some(s),
    name: "Level 2".to_owned(),
    next_level: None,
    starting_point: Vec2::new(w * 0.75, -h),
    boids_per_spawn_point: 20,
    spawn_points: vec![Vec2::new(-w / 2.0, 0.)],
    rescue_goal: 5.into(),
    time_goal: Duration::from_secs(30).into(),
    wander: false,
  };
  lvl
}
