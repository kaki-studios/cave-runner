use bevy::{prelude::*, sprite::MaterialMesh2dBundle, ecs::system::EntityCommands};
use bevy_rapier2d::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, setup_graphics, setup_physics))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Update, (print_ball_altitude, control_zoom))
        .add_plugins(RapierDebugRenderPlugin::default())
        
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    let joint = RevoluteJointBuilder::new()
    .local_anchor1(Vec2::new(0.0, 0.0))
    .local_anchor2(Vec2::new(0.00001, 0.0));

    

    /* Create the bouncing ball. */
    let mut ball: Entity = commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0))).id();

    commands.entity(ball).insert(ImpulseJoint::new(ball, joint));
        
        
}



fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    

    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    });

    // Rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    });

    // Quad
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(50., 100.)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
        transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
        ..default()
    });

    // Hexagon
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
        ..default()
    });
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}

fn control_zoom (
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut cameras: Query<&mut OrthographicProjection, With<Camera2d>>


) {
    for mut camera in cameras.iter_mut() {
        print!("Camera Scale: {}", camera.scale);

        if let Some(position) = q_windows.single().cursor_position() {
            camera.scale = position.x / (q_windows.single().physical_width() / 5) as f32 + 0.5;
        }
    }

    
}