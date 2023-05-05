//#![windows_subsystem = "windows"]

mod sprun;

use bevy::prelude::*;
use sprun::{
    consts::{SCREEN_HEIGHT, SCREEN_WIDTH},
    SpaceRunPlugin
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Space Run".to_string(),
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
                mode: WindowMode::Windowed,
                position: WindowPosition::Centered,
                resizable: false,
                cursor_visible: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(SpaceRunPlugin)
        .run();
}
