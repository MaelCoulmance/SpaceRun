/// Défini les mouvements faisable par l'utilisateur
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PlayerMove {
    Up,
    Down,
    Left,
    Right,
    None
}

/// Un événement envoyé chaque fois que l'utilisateur fais bouger
/// le vaisseau.
/// Le [`PlayerMoveEvent`] contient un [`PlayerMove`] qui indique
/// le mouvement qui vient d'être effectué par l'utilisateur
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PlayerMoveEvent(pub PlayerMove);