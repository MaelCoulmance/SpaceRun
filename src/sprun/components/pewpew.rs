use super::*;

#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PewPewStatus {
    Normal1,
    Normal2,
    Explode1,
    Explode2
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PewPewShootedEvent;

#[derive(Resource)]
pub struct PewPewLastShot(pub Instant);

#[derive(Resource)]
pub struct PewPewOnScreen(pub LinkedList<Entity>);