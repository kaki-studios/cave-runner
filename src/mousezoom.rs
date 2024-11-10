use bevy::input::mouse::*;
use bevy::prelude::*;

pub struct MouseZoomPlugin;

impl Plugin for MouseZoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, control_zoom);
    }
}

fn control_zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    mut cameras: Query<(&mut OrthographicProjection, &mut Transform), With<Camera2d>>,
) {
    for mut camera in cameras.iter_mut() {
        for ev in scroll_evr.read() {
            camera.0.scale -= ev.y / 10.0;
            if camera.0.scale < 0.0 {
                camera.0.scale = 0.0
            }
        }
    }
}
