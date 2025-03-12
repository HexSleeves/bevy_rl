#![warn(
    clippy::too_many_lines,
    clippy::if_not_else,
    clippy::explicit_iter_loop,
    clippy::explicit_into_iter_loop,
    clippy::exit,
    clippy::else_if_without_else,
    clippy::dbg_macro
)]

use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    window::{WindowMode, WindowResolution},
};

mod app_constants;
pub use self::app_constants::*;

mod app_settings;
pub use self::app_settings::*;

pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

fn main() {
    let mut app = App::new();

    // Load AppSettings
    let app_settings = AppSettings::default();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: AppConstants::APP_NAME.to_string(),
                    resolution: WindowResolution::new(
                        // app_settings.window_width(),
                        // app_settings.window_height(),
                        WINDOW_WIDTH,
                        WINDOW_HEIGHT,
                    ),
                    mode: if app_settings.fullscreen() {
                        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
                    } else {
                        WindowMode::Windowed
                    },
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(AssetPlugin {
                file_path: AppConstants::BASE.to_string(),
                meta_check: AssetMetaCheck::Never,
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .init_state::<RunningState>()
    .configure_sets(
        Update,
        (
            AppSet::RecordInput,
            AppSet::Visibility,
            AppSet::Update,
            AppSet::Render,
        )
            .chain(),
    );

    // app.insert_resource(app_settings);

    // #[cfg(feature = "dev")]
    // app.add_plugins(crate::dev::DevPlugin);

    // app.add_plugins((ControllerPlugin, ModelPlugin, UiPlugin, ViewPlugin));

    app.run();
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum AppSet {
    /// Record player input.
    RecordInput,
    /// Tick systems based on input.
    Visibility,
    /// Update - everything else goes here
    Update,
    /// Bracket-lib rendering happens here
    Render,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum RunningState {
    #[default]
    Load,
    Paused,
    Running,
}
