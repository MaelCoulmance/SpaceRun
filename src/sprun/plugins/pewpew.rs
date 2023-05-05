//use super::*;
//use std::time::{Duration, Instant};

use crate::sprun::{
    prelude::*,
    pewpew::*,
    consts::*,
    spaceship::SpaceShip,
    colisions::ColisionEvent
};




#[derive(Resource)]
struct PewPewAssets(pub HashMap<PewPewStatus, Handle<Image>>);


pub struct PewPewPlugin;

impl Plugin for PewPewPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<PewPewShootedEvent>()
            .insert_resource(PewPewAssets(HashMap::new()))
            .insert_resource(PewPewLastShot(Instant::now()))
            .insert_resource(PewPewOnScreen(LinkedList::new()))
            .add_startup_system(init_pewpew_system)
            .add_system(spawn_new_pewpew_system)
            .add_system(update_pewpew_system)
            .add_system(handle_colisions_system);
    }
}


fn init_pewpew_system (
    mut assets: ResMut<PewPewAssets>,
    server: Res<AssetServer>,
) {
    // Load assets
    let normal1: Handle<Image> = server.load("shoot/shoot1.png");
    let normal2: Handle<Image> = server.load("shoot/shoot2.png");

    let explode1: Handle<Image> = server.load("shoot/exp1.png");
    let explode2: Handle<Image> = server.load("shoot/exp2.png");


    // Store assets
    assets.0.insert(PewPewStatus::Normal1, normal1.into());
    assets.0.insert(PewPewStatus::Normal2, normal2.into());
    assets.0.insert(PewPewStatus::Explode1, explode1.into());
    assets.0.insert(PewPewStatus::Explode2, explode2.into());
}

fn spawn_new_pewpew_system(
    mut commands: Commands,
    mut reader: EventReader<PewPewShootedEvent>,
    mut last_shot: ResMut<PewPewLastShot>,
    mut entities: ResMut<PewPewOnScreen>,
    query: Query<&Transform, With<SpaceShip>>,
    assets: Res<PewPewAssets>
) {
    for _ in reader.iter() {
        if Instant::now().duration_since(last_shot.0).as_secs_f32() >= PEWPEW_DELAY {
            last_shot.0 = Instant::now();
            #[cfg(feature = "debug_pewpew")]
            println!("Just pewpewed");
            
            for pos in query.iter() {
                let id = commands.spawn(
                    (PewPewStatus::Normal1,
                            SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(PEWPEW_WIDTH, PEWPEW_HEIGHT)),
                                    ..default()
                                }, 
                                texture: assets.0.get(&PewPewStatus::Normal1).unwrap().clone().into(),
                                transform: Transform::from_xyz(pos.translation.x, pos.translation.y + PEWPEW_HEIGHT, ELEMENTS_Z),
                                ..default()
                            })
                ).id();

                entities.0.push_back(id);
            }
        }
    };
}

fn update_pewpew_system(
    mut commands: Commands,
    mut entities: ResMut<PewPewOnScreen>,
    mut queries: Query<(&mut Transform, &mut Handle<Image>, &mut PewPewStatus)>,
    assets: Res<PewPewAssets>
) {
    let mut to_be_removed = Vec::new();

    for ent in entities.0.iter() {
        let Ok((mut pos, mut texture, mut status)) = queries.get_mut(*ent) else {
            continue;
        };

        if pos.translation.y + PEWPEW_MOVE_Y > SCREEN_HEIGHT/2. {
            // Le pewpew sort de l'écran, on le supprime de l'appli.
            let Some(mut e) = commands.get_entity(*ent) else {
                #[cfg(feature = "debug_pewpew")]
                println!("Error: cannot find entity {:?} in commands", ent);
                continue;
            };

            e.despawn();
            to_be_removed.push(*ent);

            #[cfg(feature = "debug_pewpew")]
            println!("Entity {:?} just removed", ent);
        }
        else {
            // Le pewpew ne sort pas de l'écran, on met son status à jour.
            match *status {
                PewPewStatus::Normal1 => {
                    *texture = assets.0.get(&PewPewStatus::Normal2).unwrap().clone().into();
                    *status = PewPewStatus::Normal2;
                    pos.translation.y += PEWPEW_MOVE_Y;
                },

                PewPewStatus::Normal2 => {
                    *texture = assets.0.get(&PewPewStatus::Normal1).unwrap().clone().into();
                    *status = PewPewStatus::Normal1;
                    pos.translation.y += PEWPEW_MOVE_Y;
                },

                PewPewStatus::Explode1 => {
                    *texture = assets.0.get(&PewPewStatus::Explode2).unwrap().clone().into()
                },

                PewPewStatus::Explode2 => {
                    // do nothing
                }
            }
        }
    }

    entities.0 = entities.0
                    .clone()
                    .into_iter()
                    .filter(|x| !to_be_removed.contains(x))
                    .collect();

    #[cfg(feature = "debug_pewpew")]
    println!("Number of pewpew entities currently displayed: {}", entities.0.len());
}

fn handle_colisions_system(
    mut commands: Commands,
    mut entities: ResMut<PewPewOnScreen>,
    mut server: EventReader<ColisionEvent>
) {
    let mut to_be_removed = Vec::new();

    for evt in server.iter() {
        match evt {
            ColisionEvent::Enemy_SpaceShip(_) => continue,
            ColisionEvent::PewPew_Enemy(idp, _) => {
                commands.get_entity(*idp).unwrap().despawn();
                to_be_removed.push(*idp);
            }
        }
    }

    entities.0 = entities.0
                    .clone()
                    .into_iter()
                    .filter(|x| !to_be_removed.contains(x))
                    .collect();
}