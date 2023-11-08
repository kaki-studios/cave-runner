use bevy::{prelude::*, window::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use noise::core::open_simplex::*;
use noise::permutationtable::PermutationTable;
use rand::Rng;

mod raycast;
use raycast::RaycastPlugin;

mod mousezoom;
use mousezoom::MouseZoomPlugin;

mod mouseworldpos;
use mouseworldpos::*;

#[derive(Resource)]
struct VertsTest {
    verts: Vec<Vec2>,
}

#[derive(Resource)]
struct CubeQuery;

#[derive(Resource)]
struct HasherData {
    hasher: PermutationTable,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "cave-runner".into(),
                resolution: (1600., 900.).into(),
                present_mode: PresentMode::AutoVsync,
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                window_theme: Some(WindowTheme::Dark),
                // This will spawn an invisible window
                // The window will be made visible in the make_visible() system after 3 frames.
                // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                ..default()
            }),
            ..default()
        }),))
        .add_systems(Startup, setup_physics)
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            RaycastPlugin,
            MouseZoomPlugin,
            MouseWorldPos,
        ))
        .add_systems(Update, move_cube)
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(RapierContext::default())
        .run();
}

#[derive(Component)]
struct GroundMarker;

fn setup_physics(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            texture: asset_server.load("square.png"),
            ..default()
        })
        .insert(CubeMarker);

    let hasher = PermutationTable::new(rng.gen_range(0..9999));

    commands.insert_resource(HasherData { hasher: hasher });

    commands.insert_resource(VertsTest { verts: vec![] });

    commands.spawn((Camera2dBundle::default(), MainCamera));
    /* Create the ground. */
    let _ground = commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)))
        .insert(GroundMarker)
        .id();

    // let joint = RopeJointBuilder::new()
    //     .local_anchor1(Vec2::new(0.0, 0.0))
    //     .limits([0.5, 500.0])
    //     .local_anchor2(Vec2::new(0.0, 0.0));
    // println!("ground id at start {}", ground.index());

    /* Create the bouncing ball. */
    let _ball = commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(GravityScale(10.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            1000.0, 800.0, 0.0,
        )))
        .insert(PlayerMarker)
        .insert(LockedAxes::ROTATION_LOCKED_Z)
        .id();

    // commands
    //     .entity(ground)
    //     .insert(ImpulseJoint::new(ball, joint));
}

#[derive(Component)]
struct CubeMarker;

#[derive(Component)]
struct PlayerMarker;

fn move_cube(
    mut cubes: Query<&mut Transform, With<CubeMarker>>,
    mut commands: Commands,
    hasher: Res<HasherData>,
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut verts: ResMut<VertsTest>,
) {
    for mut cube in cubes.iter_mut() {
        let translation = cube.translation.clone();

        let movement_direction = cube.rotation * Vec3::X;

        cube.translation += movement_direction * 200.0 * time.delta_seconds();

        cube.rotate_z(
            open_simplex_2d::<PermutationTable>(
                [
                    translation.x as f64 / 300.0_f64,
                    translation.y as f64 / 300.0_f64,
                ],
                &hasher.hasher,
            ) as f32
                * time.delta_seconds()
                * 5.0,
        );

        gizmos.line(
            cube.translation,
            movement_direction * 100.0 + cube.translation,
            Color::LIME_GREEN,
        );

        let point_up: Vec2 = Vec2::new(-movement_direction.y, movement_direction.x) * 50.0
            + Vec2::new(cube.translation.x, cube.translation.y);
        let point_down: Vec2 = Vec2::new(movement_direction.y, -movement_direction.x) * 50.0
            + Vec2::new(cube.translation.x, cube.translation.y);

        verts.verts.push(point_up);
        verts.verts.push(point_down);

        //BAD!! dependent on framerate
        if verts.verts.len() > 1000 {
            for i in 0..verts.verts.len() {
                //remove the start of the cave, so that
                if i < verts.verts.len() - 1000 {
                    verts.verts.remove(i);
                }
            }
        }

        for i in verts.verts.iter() {
            gizmos.line_2d(Vec2::new(i.x - 0.5, i.y - 0.5), *i, Color::LIME_GREEN);
        }

        //simple verts working!
    }
}
