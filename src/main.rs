use bevy::input::mouse::MouseMotion;
use bevy_rapier2d::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy::input::mouse::MouseScrollUnit;
use noise::permutationtable::PermutationTable;
use noise::core::open_simplex::*;
use rand::Rng;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy::{
    
    prelude::*,
    window::PresentMode,
};

mod raycast;
use raycast::RaycastPlugin;

#[derive(Resource)]
struct VertsTest {
    verts: Vec<Vec2>
}


#[derive(Resource)]
struct HasherData {
    hasher: PermutationTable
}

fn main() {
    



    App::new()

        .add_plugins(
            (DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "cave-runner".into(),
                    resolution: (1000., 500.).into(),
                    present_mode: PresentMode::AutoVsync,
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
        .add_plugins((RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0), RaycastPlugin))
        .add_systems(Update, (control_zoom, move_cube))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(RapierContext::default())
        
        .run();



    

    


    

    
     
    
    
}



fn setup_physics(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    
    
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
        

    
    let hasher = PermutationTable::new(rng.gen_range(0..9999));

    
    
    
    commands.insert_resource(HasherData {
        hasher: hasher
    });

    commands.insert_resource(VertsTest {
        verts: vec![]
    });
    
      


    
    











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
    mut gizmos: Gizmos,
    mut verts: ResMut<VertsTest>
) {
    for mut cube in cubes.iter_mut() {

        
        let translation = cube.translation.clone();

        let movement_direction = cube.rotation * Vec3::X;

        cube.translation += movement_direction * 200.0 * time.delta_seconds();
        
        cube.rotate_z(open_simplex_2d::<PermutationTable>([translation.x as f64 / 300.0_f64, translation.y as f64 / 300.0_f64], &hasher.hasher) as f32 * time.delta_seconds() * 5.0);

        gizmos.line(cube.translation, movement_direction * 100.0 + cube.translation, Color::LIME_GREEN);
        
        let point_up: Vec2 = Vec2::new(-movement_direction.y, movement_direction.x) * 50.0 + Vec2::new(cube.translation.x, cube.translation.y);
        let point_down: Vec2 = Vec2::new(movement_direction.y, -movement_direction.x) * 50.0 + Vec2::new(cube.translation.x, cube.translation.y);
        

        verts.verts.push(point_up);
        verts.verts.push(point_down);

        //BAD!! dependent on framerate
        if verts.verts.len() > 10000 {
            for i in 0..verts.verts.len() {
            

                //remove the start of the cave, so that 
                if i < verts.verts.len() - 10000 {
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









// fn cast_ray(
//     rapier_context: Res<RapierContext>,

//     buttons: Res<Input<MouseButton>>,
   

// ) {



//     if buttons.just_pressed(MouseButton::Left) {
        
//         let ray_pos = Vec2::new(0.0, -200.0);
//         let ray_dir = Vec2::new(0.0, 1.0);
//         let max_toi = 400000.0;
//         let solid = true;
//         let filter = QueryFilter::default();

//         if let Some((entity, _toi)) = rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
//             // The first collider hit has the entity `entity` and it hit after
//             // the ray travelled a distance equal to `ray_dir * toi`.
//             //let hit_point = ray_pos + ray_dir * toi;
//             //println!("Entity {:?} hit at point {}", entity, hit_point);
            
//             println!("raycast id {}", entity.index())
//         }


    

//     }
// }