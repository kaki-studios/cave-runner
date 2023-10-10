use bevy::prelude::*;
use bevy::input::mouse::*;

pub struct MouseZoomPlugin;

impl Plugin for MouseZoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, control_zoom);
    }
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