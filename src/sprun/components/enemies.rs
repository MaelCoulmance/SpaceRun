use super::*;
use crate::sprun::consts::*;
use std::time::Instant;

/// Un tag pour detecter les entitées représentant un enemie
#[derive(Component)]
pub struct Enemy {
    pub bonus: u32, // Les bonus apportés au joueur lorsqu'il tue l'ennemie
    pub pv: u32     // Les points de vie de l'ennemie
}

impl Enemy {
    pub fn new(etype: EnemyType) -> Self {
        let (bonus, pv) = match etype {
            EnemyType::Small  => (10, 1),
            EnemyType::Medium => (30, 2),
            EnemyType::Big    => (100, 3)
        };

        Self {
            bonus,
            pv
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum EnemyType {
    Small,
    Medium,
    Big
}

#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum EnemyPosition {
    Small0,
    Small1,

    Medium0,
    Medium1,

    Big0,
    Big1
}

impl EnemyPosition {
    pub fn get_next(self) -> Self {
        match self {
            Self::Small0 => Self::Small1,
            Self::Small1 => Self::Small0,

            Self::Medium0 => Self::Medium1,
            Self::Medium1 => Self::Medium0,

            Self::Big0 => Self::Big1,
            Self::Big1 => Self::Big0
        }
    }
}

impl From<EnemyPosition> for EnemyType {
    fn from(value: EnemyPosition) -> Self {
        match value {
            EnemyPosition::Small0 | EnemyPosition::Small1 => Self::Small,
            EnemyPosition::Medium0 | EnemyPosition::Medium1 => Self::Medium,
            EnemyPosition::Big0 | EnemyPosition::Big1 => Self::Big
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EnemyMove {
    // Un ligne de gros, deux lignes de moyens, trois lignes de petis
    Linear  
}

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct EnemyLocation {
    pub start_x: f32,
    pub start_y: f32,

    pub end_x: f32,
    pub end_y: f32,

    pub movement: EnemyMove,
    pub sprite: EnemyPosition
}

impl EnemyLocation {
    pub fn new(mvt: EnemyMove) -> LinkedList<Self> {
        match mvt {
            EnemyMove::Linear => Self::linear()
        }
    }

    fn linear() -> LinkedList<Self> {
        let mut res = LinkedList::new();

        let pos_x = -(SCREEN_WIDTH/2.) + (ENEMY_WIDTH);
        let mut pos_y = (SCREEN_HEIGHT/2.) - (ENEMY_HEIGHT);

        let offset_y = ENEMY_HEIGHT * 1.5;

        for i in 0..ENEMY_FULL_LINE_LENGTH {
            let start_x = pos_x + (i as f32 * ENEMY_WIDTH);
            let end_x = start_x;

            let start_y = pos_y + (6. * ENEMY_HEIGHT);
            let end_y = pos_y;

            res.push_back(EnemyLocation {
                start_x, end_x,
                start_y, end_y,
                movement: EnemyMove::Linear,
                sprite: EnemyPosition::Small0
            });
        }

        pos_y -= offset_y;

        for _ in 0..2 {
            for i in 0..ENEMY_FULL_LINE_LENGTH {
                let start_x = pos_x + (i as f32 * ENEMY_WIDTH);
                let end_x = start_x;

                let start_y = pos_y + (6. * ENEMY_HEIGHT) - (i+1) as f32;
                let end_y = pos_y;

                res.push_back(EnemyLocation {
                    start_x, end_x,
                    start_y, end_y,
                    movement: EnemyMove::Linear,
                    sprite: EnemyPosition::Medium0
                });
            }

            pos_y -= offset_y;
        }

        for _ in 0..3 {
            for i in 0..ENEMY_FULL_LINE_LENGTH {
                let start_x = pos_x + (i as f32 * ENEMY_WIDTH);
                let end_x = start_x;

                let start_y = pos_y + (6. * ENEMY_WIDTH) - (i + 3) as f32;
                let end_y = pos_y;

                res.push_back(EnemyLocation {
                    start_x, end_x,
                    start_y, end_y,
                    movement: EnemyMove::Linear,
                    sprite: EnemyPosition::Big0
                });
            }

            pos_y -= offset_y;
        }

        return res;
    }
}


#[derive(Resource)]
pub struct EnemyAssets(pub HashMap<EnemyPosition, Handle<Image>>);

#[derive(Resource)]
pub struct EnemiesOnScreen(pub LinkedList<Entity>);

#[derive(Resource)]
pub struct EnemiesLastKill(pub Instant);

#[derive(Resource)]
pub struct EnemiesMove(pub LinkedList<EnemyLocation>);

#[derive(Resource)]
pub struct EnemyMoveTimer(pub Instant);

/* 
#[derive(Resource)]
pub struct EnemiesLastMove(pub Instant);

*/