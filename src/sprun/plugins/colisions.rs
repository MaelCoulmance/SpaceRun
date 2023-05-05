use crate::sprun::{
    prelude::*,
    enemies::*,
    consts::*, colisions::ColisionEvent, components::pewpew::{PewPewOnScreen, PewPewStatus}, spaceship::SpaceShip,
};

pub struct ColisionsPlugin;

impl Plugin for ColisionsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ColisionEvent>()
            .add_system(detect_colision_pewew_enemy_system)
            .add_system(detect_colision_enemy_spaceship);
    }
}


fn detect_colision_pewew_enemy_system(
    mut server: EventWriter<ColisionEvent>,

    pewpews_on_screen: Res<PewPewOnScreen>,
    enemies_on_screen: Res<EnemiesOnScreen>,

    pewpew_queries: Query<&Transform, With<PewPewStatus>>,
    enemies_queries: Query<&Transform, With<Enemy>>
) {
    for pewpew in pewpews_on_screen.0.iter() {
        for enemy in enemies_on_screen.0.iter() {
            let Ok(&p_pos) = pewpew_queries.get(*pewpew) else {
                continue;
            };

            let Ok(&e_pos) = enemies_queries.get(*enemy) else {
                continue;
            };


            if (p_pos.translation.x >= e_pos.translation.x) && (p_pos.translation.x <= e_pos.translation.x + ENEMY_WIDTH)
            && (p_pos.translation.y >= e_pos.translation.y) && (p_pos.translation.y <= e_pos.translation.y + ENEMY_HEIGHT)
            {
                server.send(ColisionEvent::PewPew_Enemy(*pewpew, *enemy));
            }
        }  
    }
}

fn detect_colision_enemy_spaceship(
    mut server: EventWriter<ColisionEvent>,

    enemies_on_screen: ResMut<EnemiesOnScreen>,
    enemies_query: Query<(&Transform, &Enemy)>,

    spacehip_on_screen: Query<&Transform, With<SpaceShip>>
) {
    for enemy in enemies_on_screen.0.iter() {
        for s_pos in spacehip_on_screen.iter() {
            let Ok((&e_pos, _)) = enemies_query.get(*enemy) else {
                continue;
            };

            if (s_pos.translation.x >= e_pos.translation.x) && (s_pos.translation.x <= e_pos.translation.x + ENEMY_WIDTH)
            && (s_pos.translation.y >= e_pos.translation.y) && (s_pos.translation.y <= e_pos.translation.y + ENEMY_HEIGHT)
            {
                server.send(ColisionEvent::Enemy_SpaceShip(*enemy));
            }
        }
    }
}