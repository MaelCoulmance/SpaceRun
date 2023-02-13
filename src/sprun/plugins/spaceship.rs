use super::*;


pub struct SpaceShipPlugin;

impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        app 
            .insert_resource(SpaceShipAssets(HashMap::new()))
            .insert_resource(SpaceShipCurrentDir(SpaceShipOrientation::Neutral))
            .insert_resource(SpaceShipCurrentPos(SpaceShipPosition::Neutral0))
            .insert_resource(SpaceShipAnimationDelay(Timer::from_seconds(0.1, TimerMode::Repeating)))
            .add_startup_system(spawn_spaceship_system)
            .add_system(update_spaceship_view_system)
            .add_system(move_spaceship_system);
    }
}


fn spawn_spaceship_system(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut assets: ResMut<SpaceShipAssets>
) {
    // Load assets
    let ship_l0: Handle<Image> = server.load("ship/ship_l0.png");
    let ship_l1: Handle<Image> = server.load("ship/ship_l1.png");
    let ship_l2: Handle<Image> = server.load("ship/ship_l2.png");
    let ship_l3: Handle<Image> = server.load("ship/ship_l3.png");

    let ship_n0: Handle<Image> = server.load("ship/ship_n0.png");
    let ship_n1: Handle<Image> = server.load("ship/ship_n1.png");

    let ship_r0: Handle<Image> = server.load("ship/ship_r0.png");
    let ship_r1: Handle<Image> = server.load("ship/ship_r1.png");
    let ship_r2: Handle<Image> = server.load("ship/ship_r2.png");
    let ship_r3: Handle<Image> = server.load("ship/ship_r3.png");


    // Store assets
    {
        use SpaceShipPosition::*;

        assets.0.insert(Left0, ship_l0.into());
        assets.0.insert(Left1, ship_l1.into());
        assets.0.insert(Left2, ship_l2.into());
        assets.0.insert(Left3, ship_l3.into());

        assets.0.insert(Neutral0, ship_n0.into());
        assets.0.insert(Neutral1, ship_n1.into());

        assets.0.insert(Right0, ship_r0.into());
        assets.0.insert(Right1, ship_r1.into());
        assets.0.insert(Right2, ship_r2.into());
        assets.0.insert(Right3, ship_r3.into());
    }

    // Spawn ship
    commands.spawn(
        (
            SpaceShip,
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(SPACESHIP_WIDTH, SPACESHIP_HEIGHT)),
                    ..default()
                },
                texture: assets.0.get(&SpaceShipPosition::Neutral0).unwrap().clone(),
                transform: Transform::from_xyz(0., -(SCREEN_HEIGHT/2.) + (SPACESHIP_HEIGHT*2.), 8.),
                ..default()
            }
        )
    );
}


fn update_spaceship_view_system(
    mut current_pos: ResMut<SpaceShipCurrentPos>,
    current_dir: Res<SpaceShipCurrentDir>,
    assets: Res<SpaceShipAssets>,
    mut queries: Query<&mut Handle<Image>, With<SpaceShip>>,
    time: Res<Time>,
    mut timer: ResMut<SpaceShipAnimationDelay>
) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut texture in queries.iter_mut() {
            let next = current_pos.0.get_next(current_dir.0);
            //println!("{:?} - {:?} - {:?}", current_dir.0, current_pos.0, next);
            *texture = assets.0.get(&next).unwrap().clone();
            current_pos.0 = next;
        }
    }
}

fn move_spaceship_system(
    mut current_dir: ResMut<SpaceShipCurrentDir>,
    mut locker: ResMut<BackgroundLocker>,
    mut events: EventReader<PlayerMoveEvent>,
    mut queries: Query<&mut Transform, With<SpaceShip>>,
) {
    for mut pos in queries.iter_mut() {
        /* 
        if events.is_empty() {
            // Pas de mouvement, le vaisseau redescent naturellement vers le bas de l'écran.
            current_dir.0 = SpaceShipOrientation::Neutral;

            if pos.translation.y - SPACESHIP_MOVE_Y > -(SCREEN_HEIGHT/2.) + (SPACESHIP_HEIGHT) {
                pos.translation.y -= SPACESHIP_MOVE_Y / 2.;
                locker.0 = false;
            } else {
                locker.0 = true;
            }

            return;
        } */

        for evt in events.iter() {
            match evt.0 {
                PlayerMove::Down => {
                    current_dir.0 = SpaceShipOrientation::Neutral;

                    if pos.translation.y - SPACESHIP_MOVE_Y > -(SCREEN_HEIGHT/2.) + (SPACESHIP_HEIGHT) {
                        pos.translation.y -= SPACESHIP_MOVE_Y;
                        locker.0 = false;
                    } else {
                        locker.0 = true;
                    }
                },
                PlayerMove::Up => {
                    current_dir.0 = SpaceShipOrientation::Neutral;

                    if pos.translation.y + SPACESHIP_MOVE_Y < 0. - SPACESHIP_HEIGHT {
                        pos.translation.y += SPACESHIP_MOVE_Y;
                        locker.0 = false;
                    } else {
                        locker.0 = true;
                    }
                },
                PlayerMove::Left => {
                    current_dir.0 = SpaceShipOrientation::Left;

                    if pos.translation.x - SPACESHIP_MOVE_X > -(SCREEN_HEIGHT/2.) + (SPACESHIP_WIDTH*2.) {
                        pos.translation.x -= SPACESHIP_MOVE_X;
                        locker.0 = false;
                    } else {
                        locker.0 = true;
                    }
                },
                PlayerMove::Right => {
                    current_dir.0 = SpaceShipOrientation::Right;

                    if pos.translation.x + SPACESHIP_MOVE_X < (SCREEN_HEIGHT/2.) - (2.* SPACESHIP_WIDTH) {
                        pos.translation.x += SPACESHIP_MOVE_X;
                        locker.0 = false;
                    } else {
                        locker.0 = true;
                    }
                }
                PlayerMove::None => {
                    // Pas de mouvement, le vaisseau redescent naturellement vers le bas de l'écran.
                    current_dir.0 = SpaceShipOrientation::Neutral;

                    if pos.translation.y - SPACESHIP_MOVE_Y > -(SCREEN_HEIGHT/2.) + (SPACESHIP_HEIGHT) {
                        pos.translation.y -= SPACESHIP_MOVE_Y / 2.;
                        locker.0 = false;
                    } else {
                        locker.0 = true;
                    }
                }
            };
        }
    }
}