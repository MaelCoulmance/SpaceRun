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


/// Système de gestion des entrées.
/// Lis les entrées clavier et les entrées souris de l'utilisateur,
/// envoie un [`PlayerMoveEvent`] à l'application si une demande de 
/// mouvement a été détectée.
fn handle_input_system(
    mut writer: EventWriter<PlayerMoveEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_input: EventReader<MouseMotion>
) {
    let mut moving = false;

    if keyboard_input.pressed(KeyCode::Up) {
        writer.send(PlayerMoveEvent(PlayerMove::Up));
        moving = true;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        writer.send(PlayerMoveEvent(PlayerMove::Down));
        moving = true;
    }
    if keyboard_input.pressed(KeyCode::Left) {
        writer.send(PlayerMoveEvent(PlayerMove::Left));
        moving = true;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        writer.send(PlayerMoveEvent(PlayerMove::Right));
        moving = true;
    }

    for evt in mouse_input.iter() {
        if evt.delta.x > 0. {
            writer.send(PlayerMoveEvent(PlayerMove::Right));
            moving = true;
        }
        if evt.delta.x < 0. {
            writer.send(PlayerMoveEvent(PlayerMove::Left));
            moving = true;
        }
        if evt.delta.y < 0. {
            writer.send(PlayerMoveEvent(PlayerMove::Up));
            moving = true;
        }
        if evt.delta.y > 0. {
            writer.send(PlayerMoveEvent(PlayerMove::Down));
            moving = true;
        }
    }

    if !moving {
        writer.send(PlayerMoveEvent(PlayerMove::None));
    }
}