use super::*;

/// Une énumeration décrivant le status actuel du projectile.
/// Cette valeur sera utilisée pour déterminée l'image à afficher
/// à l'écran
#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum PewPewStatus {
    Normal1,
    Normal2,
    Explode1,
    Explode2
}

/// Un événement permettant d'indiquer à l'application qu'un nouveau
/// projectile vient d'être tiré.
/// L'événement est déclenché par [`PlayerMovePlugin`] 
/// lorsque l'utilisateur appuie sur la touche espace ou sur le bouton gauche de la souris.
/// L'événement est consomé par le system [`spawn_new_pewpew_system`] du
/// plugin [`PewPewPlugin`].
/// 
/// [`PlayerMovePlugin`]: crate::sprun::player_move::PlayerMovePlugin
/// [`PewPewPlugin`]: crate::sprun::pewpew::PewPewPlugin
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PewPewShootedEvent;

/// Un timer pour éviter de tirer trop de projectiles à la suite.
/// Le timer contient un [`Instant`] indiquant le "time-point" auquel
/// le dernier projectile à été tiré. Lorsqu'un [`PewPewShootedEvent`]
/// est envoyé au [`PewPewPlugin`], celui-ci va vérifier que la durée
/// minimale autorisée entre deux tirs à bien été écoulée;
/// Si oui - ajoute un nouveau projectile est met à jour l'[`Instant`]
///          du [`PewPewLastShot`] avec le "time-point" [`Instant::now()`][now]
/// Si non - ignore l'évenement.
/// Le délai minimum autorisé entre deux tirs est indiqué par la constante
/// [`PEWPEW_DELAY`][delay].
/// 
/// [`PewPewPlugin`]: crate::sprun::pewpew::PewPewPlugin
/// [delay]: crate::sprun::consts::PEWPEW_DELAY
/// [now]: std::time::Instant
#[derive(Resource)]
pub struct PewPewLastShot(pub Instant);

/// Une structure contenant l'[`Entity`] de tout les projectiles
/// actuellement visibles à l'écran.
/// Cette liste est utilisée par le plugin [`PewPewPlugin`] pour
/// garder une trace de tout les projectiles, et pouvoir ainsi
/// les de-spawn lorsqu'on detecte qu'il sortent de l'écran.
/// 
/// [`PewPewPlugin`]: crate::sprun::pewpew::PewPewPlugin
#[derive(Resource)]
pub struct PewPewOnScreen(pub LinkedList<Entity>);