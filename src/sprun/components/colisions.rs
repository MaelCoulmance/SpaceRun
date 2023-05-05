use super::*;


#[derive(Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ColisionEvent {
    PewPew_Enemy(Entity, Entity),      // 0: PewPew, 1: Enemy
    Enemy_SpaceShip(Entity)            // 0: Enemy
}