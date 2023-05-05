mod background;
mod player_move;
mod spaceship;
mod pewpew;
mod enemies;
mod colisions;

pub use {
    background::BackgroundPlugin,
    player_move::PlayerMovePlugin,
    spaceship::SpaceShipPlugin,
    pewpew::PewPewPlugin,
    enemies::EnemiesPlugin,
    colisions::ColisionsPlugin
};


/// Le plugin global de l'application, inséré lors de l'initialisation de la partie
pub struct SpaceRunPlugin;

impl bevy::prelude::Plugin for SpaceRunPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_plugin(background::BackgroundPlugin)
            .add_plugin(player_move::PlayerMovePlugin)
            .add_plugin(spaceship::SpaceShipPlugin)
            .add_plugin(pewpew::PewPewPlugin)
            .add_plugin(EnemiesPlugin)
            .add_plugin(ColisionsPlugin);
    }
}