use crate::mouseworldpos::MyWorldCoords;
use crate::CubeQuery;
use crate::MainCamera;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub struct RaycastPlugin;

//TODO: Be able to cast ray towards mouse
impl Plugin for RaycastPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cast_ray);
    }
}

fn cast_ray(
    rapier_context: Res<RapierContext>,

    buttons: Res<Input<MouseButton>>,

    mousepos: Res<MyWorldCoords>,

    mut q_camera: Query<&mut Transform, With<MainCamera>>,
    mut gizmos: Gizmos,

    mut cube: Option<ResMut<CubeQuery>>,
    //mut commands: Commands
) {
    //somehow raycast doesn't work perfectly after changing camera position to follow player...
    let ray_pos = Vec2::new(
        q_camera.single().translation.x,
        q_camera.single().translation.y,
    );
    let ray_dir = mousepos.0;
    gizmos.line_2d(ray_pos, ray_dir, Color::LIME_GREEN);
    gizmos.circle_2d(mousepos.0, 5.0, Color::RED);

    if buttons.just_pressed(MouseButton::Left) {
        if let Some(mut cubelol) = cube {
            cubelol.cube.translation.x = mousepos.0.x;
            cubelol.cube.translation.y = mousepos.0.y;
            //NOT CHANGING! because we are caching cube in cubelol, so modifying cubelol won't
            //change cube
            println!("HELLO!!");
        }

        let max_toi = 4_000_000.0;
        let solid = true;
        let filter = QueryFilter::default();

        if let Some((entity, _toi)) =
            rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter)
        {
            // The first collider hit has the entity `entity` and it hit after
            // the ray travelled a distance (vector) equal to `ray_dir * toi`.
            //let hit_point = ray_pos + ray_dir * toi;
            //println!("Entity {:?} hit at point {}", entity, hit_point);

            println!("raycast id {}", entity.index())
        }
    }
}
