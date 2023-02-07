use super::*;

#[derive(Component)]
struct BackgroundSpeed(pub f32);

#[derive(Resource)]
struct BackgroundAssets(pub HashMap<i32, Handle<Image>>);

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BackgroundAssets(HashMap::new()))
            .add_startup_system(spawn_background_system)
            .add_system(move_background_system);
    }
}


fn spawn_background_system(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut assets: ResMut<BackgroundAssets>
) {
    // Spawn camera
    commands.spawn(Camera2dBundle::default());


    // Load assets
    let img0: Handle<Image> = server.load("background/img0.png");
    let img1: Handle<Image> = server.load("background/img1.png");
    let img2: Handle<Image> = server.load("background/img2.png");
    let img3: Handle<Image> = server.load("background/img3.png");
    let img4: Handle<Image> = server.load("background/img4.png");
    let img5: Handle<Image> = server.load("background/img5.png");
    let img6: Handle<Image> = server.load("background/img6.png");
    let img7: Handle<Image> = server.load("background/img7.png");

    // Initialize BackgroundAssets
    assets.0.insert(0, img0.into());
    assets.0.insert(1, img1.into());
    assets.0.insert(2, img2.into());
    assets.0.insert(3, img3.into());
    assets.0.insert(4, img4.into());
    assets.0.insert(5, img5.into());
    assets.0.insert(6, img6.into());
    assets.0.insert(7, img7.into());

    // Spawn assets
    commands.spawn(
        (
            BackgroundSpeed(0.4),
            //BackgroundMeta::default(),
            SpriteBundle {
                texture: assets.0.get(&0).unwrap().clone().into(),
                transform: Transform::from_scale(Vec3::new(0.5, 0.5, 0.)),
                ..default()
            }
        )
    );

    commands.spawn(
        (
            BackgroundSpeed(0.5),
            //BackgroundMeta::default(),
            SpriteBundle {
                texture: assets.0.get(&1).unwrap().clone().into(),
                transform: Transform::from_scale(Vec3::new(0.5, 0.5, 1.)),
                ..default()
            }
        )
    );

    commands.spawn(
        (
            BackgroundSpeed(0.6),
            //BackgroundMeta::default(),
            SpriteBundle {
                texture: assets.0.get(&2).unwrap().clone().into(),
                transform: Transform::from_scale(Vec3::new(0.5, 0.5, 2.)),
                ..default()
            }
        )
    );

    commands.spawn(
        (
            BackgroundSpeed(0.7),
            //BackgroundMeta::default(),
            SpriteBundle {
                texture: assets.0.get(&3).unwrap().clone().into(),
                transform: Transform::from_scale(Vec3::new(0.5, 0.5, 3.)),
                ..default()
            }
        )
    );

    commands.spawn(
        (
            BackgroundSpeed(0.8),
            //BackgroundMeta::default(),
            SpriteBundle {
                texture: assets.0.get(&4).unwrap().clone().into(),
                transform: Transform::from_scale(Vec3::new(0.5, 0.5, 4.)),
                ..default()
            }
        )
    );

    commands.spawn(
        (
            BackgroundSpeed(0.9),
            //BackgroundMeta::default(),
            SpriteBundle {
                texture: assets.0.get(&5).unwrap().clone().into(),
                transform: Transform::from_scale(Vec3::new(0.5, 0.5, 5.)),
                ..default()
            }
        )
    );

    commands.spawn(
        (
            BackgroundSpeed(1.),
            //BackgroundMeta::default(),
            SpriteBundle {
                texture: assets.0.get(&6).unwrap().clone().into(),
                transform: Transform::from_scale(Vec3::new(0.5, 0.5, 6.)),
                ..default()
            }
        )
    );

    commands.spawn(
        (
            BackgroundSpeed(1.1),
            //BackgroundMeta::default(),
            SpriteBundle {
                texture: assets.0.get(&7).unwrap().clone().into(),
                transform: Transform::from_scale(Vec3::new(0.5, 0.5, 7.)),
                ..default()
            }
        )
    );
}


fn move_background_system(
    mut events: EventReader<PlayerMoveEvent>,
    mut queries: Query<(&mut Transform, &BackgroundSpeed)>
) {
    for evt in events.iter() {
        for (mut pos, speed) in queries.iter_mut() {
            match evt.0 {
                PlayerMove::Down => {
                    pos.translation.y += speed.0;
                },
                PlayerMove::Up => {
                    pos.translation.y -= speed.0;
                },
                PlayerMove::Left => {
                    pos.translation.x += speed.0;
                },
                PlayerMove::Right => {
                    pos.translation.x -= speed.0;
                }
            }
        }
    }
}