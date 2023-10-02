use bevy::input::mouse::MouseMotion;
use bevy_rapier2d::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy::input::mouse::MouseScrollUnit;
use noise::permutationtable::PermutationTable;
use noise::core::perlin::*;
use noise::core::open_simplex::*;
use noise::{Fbm, OpenSimplex,};
use noise::utils::{PlaneMapBuilder, NoiseMapBuilder};
use rand::Rng;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::{
    
    prelude::*,
    window::PresentMode,
};




#[derive(Resource)]
struct HasherData {
    hasher: PermutationTable
}

fn main() {
    



    App::new()

        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "cave-runner".into(),
                    resolution: (1000., 500.).into(),
                    present_mode: PresentMode::Immediate,
                    //vsync still on?
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    
                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    transparent: false,
                    
                    ..default()
                }),
                ..default()
            }),
            
        ))
        .add_systems(Startup,  setup_physics)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Update, (control_zoom, cast_ray, move_cube))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(RapierContext::default())
        
        .run();



    

    


    

    
     
    
    
}



fn setup_physics(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    let fbm = Fbm::<OpenSimplex>::new(rng.gen_range(0..9999));
    
    
    
    let noisemap = PlaneMapBuilder::<_, 2>::new(&fbm)
            .set_size(1000, 1000)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build();
    
    println!("{}, {:?}", noisemap.size().0, noisemap.size().1);

    println!("{}", usize::MAX);
    
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        texture: asset_server.load("square.png"),
        ..default()
    }).insert(PlayerMarker);
        

    noisemap.write_to_file("fbm11.png");
    let hasher = PermutationTable::new(rng.gen_range(0..9999));
    
    
    
    commands.insert_resource(HasherData {
        hasher: hasher
    });
    
      


    for x in 0..10 {
        for y in 0..10 {

            println!("result: {}", perlin_2d::<PermutationTable>([x as f64 + 0.5_f64, y as f64 + 0.5_f64], &hasher));
        }
    }
    //TODO:fix this so that result ins't 0 so that you can get perlin values at any position possible











    commands.spawn(Camera2dBundle::default());
    /* Create the ground. */
    let ground = commands
        .spawn(RigidBody::Fixed)
        .insert(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)))
        
        .id();

    let joint = RevoluteJointBuilder::new()
    .local_anchor1(Vec2::new(-500.0, 500.0))
    .local_anchor2(Vec2::new(0.0, 0.0));
    println!("ground id at start {}", ground.index());
    

    /* Create the bouncing ball. */
    let ball: Entity = commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        
        .insert(GravityScale(10.0))
        .insert(TransformBundle::from(Transform::from_xyz(100.0, 400.0, 0.0))).id();

    commands.entity(ground).insert(ImpulseJoint::new(ball, joint));
    
    
        
}

#[derive(Component)]
struct PlayerMarker;




fn control_zoom (
    
    mut scroll_evr: EventReader<MouseWheel>,
    mut mouse_evr: EventReader<MouseMotion>,
    buttons: Res<Input<MouseButton>>,
    mut cameras: Query<(&mut OrthographicProjection, &mut Transform), With<Camera2d>>,


) {
    for mut camera in cameras.iter_mut() {
        //print!("Camera Scale: {}", camera.scale);


        

        if buttons.pressed(MouseButton::Left) {

            for ev in mouse_evr.iter() {
                
                camera.1.translation += Vec3::new(-ev.delta.x, ev.delta.y, 0.0) * camera.0.scale;
            }
        }

        


        
        for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                camera.0.scale -= ev.y / 10.0;
                if camera.0.scale < 0.0 {camera.0.scale = 0.0}
                
            }
            MouseScrollUnit::Pixel => {
                //println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
            }
        }
    }
    }

    
}

fn move_cube(


    mut cubes: Query<&mut Transform, With<PlayerMarker>>,
    hasher: Res<HasherData>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for mut cube in cubes.iter_mut() {

        
        let translation = cube.translation.clone();
        //cube.translation.y += noisemap.noise.get_value((translation.x % 1000.0).abs() as usize, (translation.y  % 1000.0).abs() as usize) as f32 * time.delta_seconds() * 200.0;
        cube.translation.y += open_simplex_2d::<PermutationTable>([translation.x as f64 / 100.0_f64, translation.y as f64 / 100.0_f64], &hasher.hasher) as f32 * time.delta_seconds() * 10000.0;
        cube.translation.x += 1000.0 * time.delta_seconds();
        //println!("{}, {:?}", (translation.x % 1000.0).abs() as usize, (translation.y  % 1000.0).abs() as usize);
        
        

        
        
        /*
        let rotation = cube.rotation.clone();     
        let translation = cube.translation.clone();

        let movement_direction = cube.rotation * Vec3::X;

        cube.translation += movement_direction * 100.0 * time.delta_seconds();
        
        
        println!("x: {}", translation.x.abs() as usize);
        println!("y: {}", translation.y.abs() as usize);

        
        
        
        
        cube.rotate_z(noisemap.noise[(((translation.x + 500.0) % 1000.0).abs() as usize, ((translation.y + 500.0) % 1000.0).abs() as usize)] as f32 * 0.05);

        gizmos.line(cube.translation, movement_direction * 100.0 + cube.translation, Color::LIME_GREEN);
        
         */
        

        // Circle
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(500.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::MIDNIGHT_BLUE)),
            transform: Transform::from_translation(cube.translation),
            ..default()
        });
    }

}

pub fn direction(rotation_angle: f32) -> Vec3 {
        let (y, x) = (rotation_angle).sin_cos();

        Vec3::new(x, y, 0.0).normalize()
}








fn cast_ray(
    rapier_context: Res<RapierContext>,

    buttons: Res<Input<MouseButton>>,
   

) {



    if buttons.just_pressed(MouseButton::Left) {
        
        let ray_pos = Vec2::new(0.0, -200.0);
        let ray_dir = Vec2::new(0.0, 1.0);
        let max_toi = 400000.0;
        let solid = true;
        let filter = QueryFilter::default();

        if let Some((entity, _toi)) = rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
            // The first collider hit has the entity `entity` and it hit after
            // the ray travelled a distance equal to `ray_dir * toi`.
            //let hit_point = ray_pos + ray_dir * toi;
            //println!("Entity {:?} hit at point {}", entity, hit_point);
            
            println!("raycast id {}", entity.index())
        }


    

    }
}
