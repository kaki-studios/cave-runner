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
    ball_query: Query<(&Transform, Entity), (With<PlayerMarker>, Without<MainCamera>)>,
    mut commands: Commands,
    ground_query: Query<
        (Entity, Option<&mut ImpulseJoint>),
        (
            With<GroundMarker>,
            Without<MainCamera>,
            Without<PlayerMarker>,
        ),
    >,
) {
    //somehow raycast doesn't work perfectly after changing camera position to follow player...
    let ray_pos = Vec2::new(
        ball_query.single().0.translation.x,
        ball_query.single().0.translation.y,
    );
    let ray_dir = mousepos.0;
    gizmos.line_2d(ray_pos, ray_dir, Color::LIME_GREEN);
    gizmos.circle_2d(mousepos.0, 5.0, Color::RED);

    if buttons.just_pressed(MouseButton::Left) {
        let filter: QueryFilter = QueryFilter::only_fixed();
        // filter.exclude_collider(ball_query.single().1);

        let max_toi = 4_000_000.0;
        let solid = false;

        if let Some((entity, _toi)) =
            rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter)
        {
            if entity == ground_query.single().0 {
                if let Some(_ground) = ground_query.single().1 {
                    commands.entity(entity).remove::<ImpulseJoint>();
                } else {
                    let hit_point = ray_pos + ray_dir * _toi;
                    let joint = RopeJointBuilder::new()
                        .local_anchor1(Vec2::ZERO)
                        .limits([0.5, 500.0])
                        .local_anchor2(Vec2::ZERO);

                    commands
                        .entity(entity)
                        .insert(ImpulseJoint::new(ball_query.single().1, joint));
                }
            }
            println!(
                "entity == ground query: {}",
                entity == ground_query.single().0
            );
            // The first collider hit has the entity `entity` and it hit after
            // the ray travelled a distance (vector) equal to `ray_dir * toi`.

            //println!("raycast id {}", entity.index())
        }
    }
}
