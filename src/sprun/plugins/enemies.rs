use std::time::Instant;

use crate::sprun::{
    prelude::*,
    enemies::*,
    consts::*, colisions::ColisionEvent,
};

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(EnemyAssets(HashMap::new()))
            .insert_resource(EnemiesOnScreen(LinkedList::new()))
            .insert_resource(EnemiesLastKill(Instant::now()))
            .insert_resource(EnemyMoveTimer(Instant::now()))
            //.insert_resource(EnemiesMove(LinkedList::new()))
            //.insert_resource(EnemiesLastMove(Instant::now()))
            .add_startup_system(load_enemies_system)
            .add_system(spawn_enemies_system)
            .add_system(update_enemies_system)
            .add_system(handle_colisions_system);
    }
}


fn load_enemies_system(
    server: Res<AssetServer>,
    mut assets: ResMut<EnemyAssets>
) {
    // Load assets
    let small0: Handle<Image> = server.load("enemies/esmall1.png");
    let small1: Handle<Image> = server.load("enemies/esmall2.png");

    let med0: Handle<Image> = server.load("enemies/emed1.png");
    let med1: Handle<Image> = server.load("enemies/emed2.png");

    let big0: Handle<Image> = server.load("enemies/ebig1.png");
    let big1: Handle<Image> = server.load("enemies/ebig2.png");


    // Store assets
    {
        use EnemyPosition::*;

        assets.0.insert(Small0, small0.into());
        assets.0.insert(Small1, small1.into());

        assets.0.insert(Medium0, med0.into());
        assets.0.insert(Medium1, med1.into());

        assets.0.insert(Big0, big0.into());
        assets.0.insert(Big1, big1.into());
    }
}

fn spawn_enemies_system(
    mut commands: Commands,
    /*mut*/ last_kill: ResMut<EnemiesLastKill>,
    mut entities: ResMut<EnemiesOnScreen>,
    //mut moves: ResMut<EnemiesMove>,
    //query: Query<&Transform, With<Enemy>>,
    assets: Res<EnemyAssets>
) {
    if entities.0.is_empty() {
        // Il n'y a aucun enemie à l'écran, on affiche un nouvel
        // essaim 2 secondes après la mort du dernier enemie du 
        // précédent essaim.
        if Instant::now().duration_since(last_kill.0).as_secs_f32() >= ENEMY_SPAWN_DELAY {
            let m = EnemyLocation::new(EnemyMove::Linear);

            for elt in m.iter() {
                let id = commands.spawn(
                    (Enemy::new(EnemyType::from(elt.sprite)),
                    elt.sprite,
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(ENEMY_WIDTH, ENEMY_HEIGHT)),
                            ..default()
                        },
                        texture: assets.0.get(&elt.sprite).unwrap().clone().into(),
                        transform: Transform::from_xyz(elt.start_x, elt.start_y, ELEMENTS_Z),
                        ..default()
                    },
                    *elt)
                ).id();

                entities.0.push_back(id);
            }
        }
    }
}

fn update_enemies_system(
    //mut commands: Commands,
    //mut entities: ResMut<EnemiesOnScreen>,
    mut queries: Query<(&mut Transform, &mut Handle<Image>, &mut EnemyPosition, &EnemyLocation), With<Enemy>>,
    //mut moves: ResMut<EnemiesMove>,
    assets: Res<EnemyAssets>,
    mut move_timer: ResMut<EnemyMoveTimer>,
) {
    // On met à jour la position des enemies.

    let lock_timer = Instant::now().duration_since(move_timer.0).as_secs_f32();
    let mut need_reset_timer = false;
    
    for (mut t, mut h, mut p, l) in queries.iter_mut() {
        if t.translation.x < l.end_x {
            t.translation.x += ENEMY_MOVE_OFFSET;
        }
        if t.translation.y > l.end_y {
            t.translation.y -= ENEMY_MOVE_OFFSET;
        }

        if lock_timer >= ENEMY_MOVE_DELAY {
            *p = EnemyPosition::get_next(*p);
            *h = assets.0.get(&p).unwrap().clone().into();

            need_reset_timer = true;
        }
    }

    if need_reset_timer {
        move_timer.0 = Instant::now();
    }
}


fn handle_colisions_system(
    mut commands: Commands,
    mut entities: ResMut<EnemiesOnScreen>,
    mut server: EventReader<ColisionEvent>,
    mut last_kill: ResMut<EnemiesLastKill>
) {
    let mut to_be_removed = Vec::new();

    for evt in server.iter() {
        match evt {
            ColisionEvent::Enemy_SpaceShip(_) => continue,
            ColisionEvent::PewPew_Enemy(_, ide) => {
                commands.get_entity(*ide).unwrap().despawn();
                to_be_removed.push(*ide);
            }
        }
    }

    entities.0 = entities.0
                    .clone()
                    .into_iter()
                    .filter(|x| !to_be_removed.contains(x))
                    .collect();


    if entities.0.is_empty() {
        last_kill.0 = Instant::now();
    }
}