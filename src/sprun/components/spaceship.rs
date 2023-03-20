use super::*;

/// Un tag pour détecter les entitées représentant un vaisseau
#[derive(Component)]
pub struct SpaceShip;

/// Une énumeration indiquant l'orientation actuelle du vaisseau.
/// Cette énumération est utilisée de pair avec [`SpaceShipPosition`]
/// pour animer le vaisseau lors de ses mouvements.
/// [`SpaceShipOrientation`] represente l'orientation globale du vaisseau,
/// indépendemment de l'image utilisée pour dessiner son [`Sprite`]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SpaceShipOrientation {
    Left,
    Neutral, 
    Right
}

/// Un énumeration indiquant l'image actuellement utilisée pour dessiner
/// le [`Sprite`] relatif à un [`SpaceShip`].
/// Cette énumération est utilisée de pair avec [`SpaceShipOrientation`]
/// pour animer le vaisseau lors de ses mouvements.
/// [`SpaceShipPosition`] représente l'orientation actuelle du vaisseau,
/// et ainsi choisir la bonne image pour animer ce dernier.
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
    /// Determine l'image utilisée pour dessiner un [`SpaceShip`] lors de
    /// la prochaine sprite, en fonction de l'image actuelle et de la
    /// nouvelle orientation du vaisseau
    /// 
    /// ### Parametres :
    /// - [`self`]() : La position actuelle du vaisseau à l'écran
    /// - [`orientation`]() : L'orientation du vaisseau lors de la prochaine
    ///   sprite
    /// 
    /// ### Valeur de retour:
    /// 
    /// La [`SpaceShipPosition`] utilisée lors de la prochaine frame 
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


/// Un dictionnaire contenant les images utilisées pour représenter un [`SpaceShip`]
/// à l'écran
/// 
/// - clés: des [`SpaceShipPosition`]
/// - valeurs: l'image correspondant à la position du vaisseau
#[derive(Resource)]
pub struct SpaceShipAssets(pub HashMap<SpaceShipPosition, Handle<Image>>);

/// Une ressource permettant de garder en mémoire l'orientation actuelle du vaisseau
#[derive(Resource)]
pub struct SpaceShipCurrentDir(pub SpaceShipOrientation);

/// Une ressource permettant de garder en mémoire la position actuelle du vaisseau
#[derive(Resource)]
pub struct SpaceShipCurrentPos(pub SpaceShipPosition);

/// Le framerate de l'animation du vaisseau
/// Le framerate est initialisée avec la constante [`SPACESHIP_DELAY`][delay]
/// 
/// [delay]: crate::sprun::consts::SPACESHIP_DELAY
#[derive(Resource)]
pub struct SpaceShipAnimationDelay(pub Timer);