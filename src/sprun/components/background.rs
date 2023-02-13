use super::*;



/// Un locker pour le fond d'écran.
/// On décide que le vaisseau est limité dans ses mouvements.
/// Il ne peut pas sortir de l'écran et ne peut pas aller plus loins
/// que le millieu de ce dernier. Lorsque le joueur se retrouve bloqué
/// à une de ses limites, il faut donc arrêter de faire défiler le fond.
/// Or, comme le mouvement du vaisseau et le mouvement des differents 
/// layers du fond d'écran sont gérés par deux systèmes différents, il
/// faut qu'on utilise une resource commune pour que ces derniers puissent
/// communiquer entre eux.
/// Un [`BackgroundLocker`] est simplement un wrapper de booleen.
/// Lorsque le système [`move_spaceship_system`] détecte que le vaisseau
/// ne peux plus avancer dans une direction, il "verouille le locker". 
/// Lorsqu'il detecte qu'un mouvement est de nouveau possible, il le
/// "deverouille".
/// De son côté, le système [`move_background_system`] vérifie simplement
/// si le locker est vérouiller ou non avant de déplacer les layers.
/// 
#[derive(Resource, Copy, Clone, Debug, PartialEq, Eq)]
pub struct BackgroundLocker(pub bool);