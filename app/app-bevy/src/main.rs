mod helper;
mod logics;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    sprite::collide_aabb::collide,
};

fn main() {
    App::build()
        .add_plugin(LogDiagnosticsPlugin {
            debug: false,
            wait_duration: std::time::Duration::from_secs(60),
            filter: None,
        })
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::DEBUG,
            filter: "wgpu_core=error".to_string(),
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            width: 900.0,
            height: 700.0,
            title: String::from("Shorelark"),
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::hex("1f2639").unwrap()))
        .add_plugins(DefaultPlugins)
        .add_plugin(logics::SimulationPlugin)
        .add_startup_system(setup.system())
        .run()
}

fn setup(mut commands: Commands) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
