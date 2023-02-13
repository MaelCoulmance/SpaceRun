use super::*;

#[derive(Component)]
pub struct SpaceShip;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SpaceShipOrientation {
    Left,
    Neutral, 
    Right
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SpaceShipPosition {
    Neutral0,
    Neutral1,

    Left0,
    Left1,
    Left2,
    Left3,

    Right0,
    Right1,
    Right2,
    Right3
}

impl SpaceShipPosition {
    pub fn get_next(self, orientation: SpaceShipOrientation) -> Self {
        use SpaceShipOrientation::*;
        
        match (self, orientation) {
            (Self::Neutral0, Neutral) => Self::Neutral1,
            (Self::Neutral1, Neutral) => Self::Neutral0,

            (Self::Left0, Left) => Self::Left1,
            (Self::Left1, Left) => Self::Left2,
            (Self::Left2, Left) => Self::Left3,
            (Self::Left3, Left) => Self::Left2,

            (Self::Right0, Right) => Self::Right1,
            (Self::Right1, Right) => Self::Right2,
            (Self::Right2, Right) => Self::Right3,
            (Self::Right3, Right) => Self::Right2,

            
            (Self::Left0, _) => Self::Neutral0,
            (Self::Left1, _) => Self::Left0,
            (Self::Left2, _) => Self::Left1,
            (Self::Left3, _) => Self::Left2,

            (Self::Right0, _) => Self::Neutral0,
            (Self::Right1, _) => Self::Right0,
            (Self::Right2, _) => Self::Right1,
            (Self::Right3, _) => Self::Right2,

            (_, Left) => Self::Left0,
            (_, Right) => Self::Right0
        }
    }
}

#[derive(Resource)]
pub struct SpaceShipAssets(pub HashMap<SpaceShipPosition, Handle<Image>>);

#[derive(Resource)]
pub struct SpaceShipCurrentDir(pub SpaceShipOrientation);

#[derive(Resource)]
pub struct SpaceShipCurrentPos(pub SpaceShipPosition);

#[derive(Resource)]
pub struct SpaceShipAnimationDelay(pub Timer);