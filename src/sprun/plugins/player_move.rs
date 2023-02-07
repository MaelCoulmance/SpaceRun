//! Un plugin qui permet de detecter les entrées de l'utilisateur.
//! Ce module définie un événement [`PlayerMoveEvent`] qui seras déclenché
//! à chaque fois que le joueur fais bouger son vaisseau (soit avec les touches
//! du clavier, soit avec la souris).
use bevy::input::mouse::MouseMotion;

use super::*;


/// Plugin qui gère la lecture des entrées de l'utilisateur
pub struct PlayerMovePlugin;

impl Plugin for PlayerMovePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PlayerMoveEvent>()
            .add_system(handle_input_system);
    }
}


/// Défini les mouvements faisable par l'utilisateur
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PlayerMove {
    Up,
    Down,
    Left,
    Right
}

/// Un événement envoyé chaque fois que l'utilisateur fais bouger
/// le vaisseau.
/// Le [`PlayerMoveEvent`] contient un [`PlayerMove`] qui indique
/// le mouvement qui vient d'être effectué par l'utilisateur
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PlayerMoveEvent(pub PlayerMove);



/// Système de gestion des entrées.
/// Lis les entrées clavier et les entrées souris de l'utilisateur,
/// envoie un [`PlayerMoveEvent`] à l'application si une demande de 
/// mouvement a été détectée.
fn handle_input_system(
    mut writer: EventWriter<PlayerMoveEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_input: EventReader<MouseMotion>
) {
    if keyboard_input.pressed(KeyCode::Up) {
        writer.send(PlayerMoveEvent(PlayerMove::Up));
    }
    if keyboard_input.pressed(KeyCode::Down) {
        writer.send(PlayerMoveEvent(PlayerMove::Down));
    }
    if keyboard_input.pressed(KeyCode::Left) {
        writer.send(PlayerMoveEvent(PlayerMove::Left));
    }
    if keyboard_input.pressed(KeyCode::Right) {
        writer.send(PlayerMoveEvent(PlayerMove::Right));
    }

    for evt in mouse_input.iter() {
        if evt.delta.x > 0. {
            writer.send(PlayerMoveEvent(PlayerMove::Right));
        }
        if evt.delta.x < 0. {
            writer.send(PlayerMoveEvent(PlayerMove::Left));
        }
        if evt.delta.y < 0. {
            writer.send(PlayerMoveEvent(PlayerMove::Up));
        }
        if evt.delta.y > 0. {
            writer.send(PlayerMoveEvent(PlayerMove::Down));
        }
    }
}