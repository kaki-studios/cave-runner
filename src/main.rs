use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy::input::mouse::MouseScrollUnit;
use noise::{RidgedMulti, Worley, Abs, Fbm, Perlin, OpenSimplex};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};



fn main() {
    let fbm = Fbm::<OpenSimplex>::new(509435);

    let mut noisemap = PlaneMapBuilder::<_, 2>::new(&fbm)
            .set_size(1000, 1000)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build();


    for mut i in &mut noisemap {
        //TODO: not modifying anything !!! probably need to mody the value at noisemap[x][y]...
        //shit!!! dont do it like this, this results in a cave SYSTEM, you only need 1 tunnel, 
        //so use the noisemap to offset a direction 
        //and that way you only have 1 tunnel


        //youtube quote: 
        //The solution to generating caves.
        //Takes its current XYZ coordinate and generates 3 octaves of perlin noise in order to determine an xyz offset to move towards. 
        //Then it shifts to that offset and places a cell, and repeats that cycle every frame.
        //this will also work better than premade maps for infinite generation
        if i < &mut -0.25 || i > &mut 0.25 {
            i = &mut 1.0;
            println!("{}", 1);
            
        }
        else {
            i = &mut 0.0;
            println!("{}", 0);
        }
        
    }

    noisemap.write_to_file("fbm9.png");



    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup,  setup_physics)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Update, (control_zoom, cast_ray))
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(RapierContext::default())
        
        .run();



    

    


    

    
     
    
    
}



fn setup_physics(mut commands: Commands) {
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
