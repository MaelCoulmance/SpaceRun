mod plugins;
mod components;

pub use bevy::prelude::*;
pub use std::collections::{HashMap, LinkedList};
pub use plugins::*;
pub use components::*;


pub const SCREEN_WIDTH: f32 = 600.;
pub const SCREEN_HEIGHT: f32 = 700.;

pub const SPACESHIP_WIDTH: f32 = 28.;
pub const SPACESHIP_HEIGHT: f32 = 48.;

pub const SPACESHIP_MOVE_X: f32 = 2.3;
pub const SPACESHIP_MOVE_Y: f32 = 2.3;