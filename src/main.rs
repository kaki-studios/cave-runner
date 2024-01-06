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
struct VertsResource {
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
            RapierDebugRenderPlugin::default(),
        ))
        .init_resource::<VertsResource>()
        .add_systems(Update, move_cube)
        .insert_resource(VertTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(RapierContext::default())
        .run();
}

fn setup_physics(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();

    //TODO: attach a sensor and win the game when the player touches it
    let _cube = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.))
                .with_rotation(Quat::from_rotation_z(-0.5 * std::f32::consts::PI)),

            texture: asset_server.load("square.png"),
            ..default()
        })
        .insert(CubeMarker)
        .id();

    let hasher = PermutationTable::new(rng.gen_range(0..9999));

    commands.insert_resource(HasherData { hasher });
    commands.spawn((Camera2dBundle::default(), MainCamera));

    /* Create the bouncing ball. ACtually this is the player! */
    let _ball = commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(30.0))
        .insert(Restitution::coefficient(0.5))
        .insert(GravityScale(1.0))
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

            transform: Transform::from_translation(Vec3::new(0., 1000., 0.)),

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
    mut verts: ResMut<VertsResource>,
    mut vert_time: ResMut<VertTimer>,
    player: Query<&Velocity, With<PlayerMarker>>,
) {
    for mut cube in cubes.iter_mut() {
        let translation = cube.translation;

        let movement_direction = cube.rotation * Vec3::X;

        //velocity never goes below 200
        //this presents a new problem:
        //the player will, over time, fall out from the end
        //because they will never catch up
        let velocity = player.single().linvel.length();
        // cube.translation += movement_direction * max(velocity, 200.0) * time.delta_seconds();
        // NOTE: See https://excalidraw.com/#json=WDUMPDCcBqB9z2h7YNwHV,pTGgmhEXSQNG5B8z4AneHw
        //TODO: fix this (and the deletion of verts) so that the player never sees either end of
        //the cave
        //NOTE: shouldnt really use velocity here (look at first comment)
        cube.translation += movement_direction * velocity * time.delta_seconds();
        let turniness = 2.5 * velocity / 300.0;

        cube.rotate_z(
            open_simplex_2d::<PermutationTable>(
                [
                    translation.x as f64 / 500.0_f64,
                    translation.y as f64 / 500.0_f64,
                ],
                &hasher.hasher,
            ) as f32
                * time.delta_seconds()
                * turniness,
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
            if verts.verts.len() > 8 {
                for _ in 0..4 {
                    //remove all verts except the ones needed for current mesh gen!
                    verts.verts.remove(0);
                }
            }
            println!("verts len {}", verts.verts.len());
        }

        //verts working!
    }
}
