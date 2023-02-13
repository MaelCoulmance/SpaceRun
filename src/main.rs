mod sprun;
use sprun::*;

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
        .add_plugin(BackgroundPlugin)
        .add_plugin(PlayerMovePlugin)
        .add_plugin(SpaceShipPlugin)
        .add_plugin(PewPewPlugin)
        .run();
}
