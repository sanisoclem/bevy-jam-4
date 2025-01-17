use bevy::prelude::*;
use bevy_hanabi::HanabiPlugin;
use game::GameExtensions;
use jam4::Jam4Extensions;
use jukebox::JukeboxExtensions;
use splash::SplashExtensions;

#[cfg(feature = "debug")]
use bevy_egui::EguiPlugin;
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum AppState {
  #[default]
  Splash,
  Game,
}

mod game;
mod jukebox;
mod splash;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

//mod audio;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn main_wasm() {
  main();
}

fn main() {
  let mut app = App::new();
  app
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        canvas: Some("#main-canvas".into()),
        ..default()
      }),
      ..default()
    }))
    .add_plugins((utils::text::TextAnimationPlugin, utils::music::MusicPlugin))
    .add_plugins(HanabiPlugin)
    .add_state::<AppState>()
    .add_splash_screen(AppState::Splash, AppState::Game)
    .add_jukebox()
    .add_jam_game()
    .add_game(AppState::Game);

  #[cfg(feature = "debug")]
  app.add_plugins((
    EguiPlugin,
    utils::fps::ScreenDiagsTextPlugin,
    WorldInspectorPlugin::default(),
  ));

  app.run();
}
