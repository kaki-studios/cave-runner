#![windows_subsystem = "windows"]
//this will disable the console from appearing on top of the game in windows
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

mod mesh_gen;
use mesh_gen::MeshGenPlugin;

#[derive(Component)]
struct CubeMarker;

#[derive(Component)]
struct PlayerMarker;

#[derive(Resource, Default)]
struct VertsTest {
    verts: Vec<Vec2>,
}

#[derive(Component)]
struct GroundMarker;

#[derive(Resource)]
struct CubeQuery;

#[derive(Resource)]
struct HasherData {
    hasher: PermutationTable,
}

#[derive(Resource)]
struct VertTimer(Timer);

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
            MeshGenPlugin,
        ))
        .init_resource::<VertsTest>()
        .add_systems(Update, move_cube)
        .insert_resource(VertTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        // .add_plugins(RapierDebugRenderPlugin::default())
        // .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(RapierContext::default())
        .run();
}

fn setup_physics(mut commands: Commands, asset_server: Res<AssetServer>, time: Res<Time>) {
    let mut rng = rand::thread_rng();

    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(0.0, 0.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.))
                .with_rotation(Quat::from_rotation_z(-0.5 * std::f32::consts::PI)),

            texture: asset_server.load("square.png"),
            ..default()
        })
        .insert(CubeMarker);

    let hasher = PermutationTable::new(rng.gen_range(0..9999));

    commands.insert_resource(HasherData { hasher });
    commands.spawn((Camera2dBundle::default(), MainCamera));

    /* Create the bouncing ball. */
    let _ball = commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(30.0))
        .insert(Restitution::coefficient(0.7))
        .insert(GravityScale(0.0))
        // .insert(TransformBundle::from(Transform::from_xyz(0.0, 800.0, 0.0)))
        .insert(Velocity::zero())
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        })
        .insert(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },

            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),

            texture: asset_server.load("character.png"),
            ..default()
        })
        .insert(PlayerMarker)
        // .insert(LockedAxes::ROTATION_LOCKED_Z)
        .id();
}

fn move_cube(
    mut cubes: Query<&mut Transform, With<CubeMarker>>,
    hasher: Res<HasherData>,
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut verts: ResMut<VertsTest>,
    mut vert_time: ResMut<VertTimer>,
) {
    for mut cube in cubes.iter_mut() {
        let translation = cube.translation;

        let movement_direction = cube.rotation * Vec3::X;

        cube.translation += movement_direction * 200.0 * time.delta_seconds();

        cube.rotate_z(
            open_simplex_2d::<PermutationTable>(
                [
                    translation.x as f64 / 500.0_f64,
                    translation.y as f64 / 500.0_f64,
                ],
                &hasher.hasher,
            ) as f32
                * time.delta_seconds()
                * 2.5,
        );

        // gizmos.line(
        //     cube.translation,
        //     movement_direction * 100.0 + cube.translation,
        //     Color::LIME_GREEN,
        // );
        if vert_time.0.tick(time.delta()).just_finished() {
            let point_high: Vec2 = Vec2::new(-movement_direction.y, movement_direction.x) * 200.0
                + cube.translation.truncate();
            let point_low: Vec2 = Vec2::new(movement_direction.y, -movement_direction.x) * 200.0
                + cube.translation.truncate();
            let point_higher = Vec2::new(-movement_direction.y, movement_direction.x) * 350.0
                + cube.translation.truncate();
            let point_lower: Vec2 = Vec2::new(movement_direction.y, -movement_direction.x) * 350.0
                + cube.translation.truncate();

            verts.verts.push(point_high);
            verts.verts.push(point_low);

            verts.verts.push(point_higher);
            verts.verts.push(point_lower);
        }

        if verts.verts.len() > 100 {
            for _ in 0..4 {
                //remove the start of the cave, so that the cave doesn't get too long
                verts.verts.remove(0);
            }
        }

        //we draw a line between each vertex!

        //simple verts working!
    }
}
