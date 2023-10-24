use crate::mouseworldpos::MyWorldCoords;
use crate::{GroundMarker, MainCamera, PlayerMarker};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub struct RaycastPlugin;

//TODO: Query all the joints and 1. disable / enable them on mouse click
//                               2. set the anchor of the point to the entity clicked
//                               and more specifically, to the correct place (in the local space of
//                               the 2nd entity
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
    mut joints: Query<Entity, With<ImpulseJoint>>,
    mut gizmos: Gizmos,
    ball_query: Query<&Transform, (With<PlayerMarker>, Without<MainCamera>)>,
    mut commands: Commands,
    ground_query: Query<
        Entity,
        (
            With<GroundMarker>,
            Without<MainCamera>,
            Without<PlayerMarker>,
        ),
    >,
) {
    //somehow raycast doesn't work perfectly after changing camera position to follow player...
    let ray_pos = Vec2::new(
        ball_query.single().translation.x,
        ball_query.single().translation.y,
    );
    let ray_dir = mousepos.0;
    gizmos.line_2d(ray_pos, ray_dir, Color::LIME_GREEN);
    gizmos.circle_2d(mousepos.0, 5.0, Color::RED);

    if buttons.just_pressed(MouseButton::Left) {
        let max_toi = 4_000_000.0;
        let solid = true;
        let filter = QueryFilter::default();

        if let Some((entity, _toi)) =
            rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter)
        {
            //TODO: this doesn't work!!
            if entity == ground_query.single() {
                commands.entity(entity).remove::<ImpulseJoint>();
            }

            // The first collider hit has the entity `entity` and it hit after
            // the ray travelled a distance (vector) equal to `ray_dir * toi`.
            //let hit_point = ray_pos + ray_dir * toi;
            //println!("Entity {:?} hit at point {}", entity, hit_point);

            println!("raycast id {}", entity.index())
        }
    }
}
