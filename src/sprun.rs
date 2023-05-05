mod plugins;
mod components;

mod prelude {
    pub use {
        bevy::prelude::*,
        std::collections::{HashMap, LinkedList},
        std::time::Instant,
    };

//pub use bevy::prelude::*;
//pub use std::collections::{HashMap, LinkedList};
//pub use std::time::Instant;
//pub use plugins::*;
//pub use components::*;
}

mod background {
    pub use super::plugins::BackgroundPlugin;
    pub use super::components::background::*;
}

mod pewpew {
    pub use super::plugins::PewPewPlugin;
    pub use super::components::pewpew::*;
}

mod player_move {
    pub use super::plugins::PlayerMovePlugin;
    pub use super::components::player_moves::*;
}

mod spaceship {
    pub use super::plugins::SpaceShipPlugin;
    pub use super::components::spaceship::*;
}

mod enemies {
    pub use super::plugins::EnemiesPlugin;
    pub use super::components::enemies::*;
}

mod colisions {
    pub use super::plugins::ColisionsPlugin;
    pub use super::components::colisions::*;
}


pub mod consts {
    pub const SCREEN_WIDTH: f32 = 600.;
    pub const SCREEN_HEIGHT: f32 = 700.;

    pub const SPACESHIP_WIDTH: f32 = 28.;
    pub const SPACESHIP_HEIGHT: f32 = 48.;

    pub const SPACESHIP_MOVE_X: f32 = 2.3;
    pub const SPACESHIP_MOVE_Y: f32 = 2.3;

    pub const SPACESHIP_DELAY: f32 = 0.1;

    pub const PEWPEW_WIDTH: f32 = 10.;
    pub const PEWPEW_HEIGHT: f32 = 24.;

    pub const PEWPEW_MOVE_Y: f32 = 2.3;

    pub const PEWPEW_DELAY: f32 = 0.2;

    pub const ELEMENTS_Z: f32 = 8.;

    pub const ENEMY_SPAWN_DELAY: f32 = 2.;

    pub const ENEMY_FULL_LINE_LENGTH: usize = (SCREEN_WIDTH / ENEMY_WIDTH) as usize - 1;
    //pub const ENEMY_HALF_LINE_LENGTH: usize = ENEMY_FULL_LINE_LENGTH / 2;
    //pub const ENEMY_QUARTER_LINE_LENGTH: usize = ENEMY_HALF_LINE_LENGTH / 2;

    pub const ENEMY_WIDTH: f32 = 30.;
    pub const ENEMY_HEIGHT: f32 = 25.;

    pub const ENEMY_MOVE_DELAY: f32 = 0.1;

    pub const ENEMY_MOVE_OFFSET: f32 = 1.5;
}

pub use plugins::SpaceRunPlugin;