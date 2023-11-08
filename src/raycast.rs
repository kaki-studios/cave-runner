use crate::mouseworldpos::MyWorldCoords;
use crate::{GroundMarker, MainCamera, PlayerMarker};
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
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

    mousepos: Res<MyWorldCoords>,

    // mut q_camera: Query<&mut Transform, With<MainCamera>>,
    mut gizmos: Gizmos,
    ball_query: Query<(&Transform, Entity), (With<PlayerMarker>, Without<MainCamera>)>,
    mut commands: Commands,
    ground_query: Query<
        (Entity, Option<&mut ImpulseJoint>, &Transform),
        (
            With<GroundMarker>,
            Without<MainCamera>,
            Without<PlayerMarker>,
        ),
    >,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
) {
    //somehow raycast doesn't work perfectly after changing camera position to follow player...
    let ray_pos = Vec2::new(
        ball_query.single().0.translation.x,
        ball_query.single().0.translation.y + 50.5,
    );
    let ray_dir = mousepos.0;
    gizmos.line_2d(ray_pos, ray_dir, Color::LIME_GREEN);
    gizmos.circle_2d(mousepos.0, 5.0, Color::RED);

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                // let filter: QueryFilter = QueryFilter::only_fixed();
                //
                // let max_toi = 4_000_000.0;
                // let solid = true;
                //
                // if let Some((entity, _toi)) =
                //     rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter)
                // {
                //     // if entity == ground_query.single().0 {
                //     // let hit_point = ray_pos + ray_dir * _toi;
                //
                //     let joint = RopeJointBuilder::new()
                //         .local_anchor2(
                //             mousepos.0 - ground_query.single().2.translation.truncate(), // Vec2::ZERO,
                //         )
                //         .limits([
                //             0.5,
                //             ground_query
                //                 .single()
                //                 .2
                //                 .translation
                //                 .distance(ball_query.single().0.translation),
                //             // 500.0,
                //         ])
                //         .local_anchor1(ray_pos - ball_query.single().0.translation.truncate());
                //
                //     commands
                //         .entity(entity)
                //         .insert(ImpulseJoint::new(ball_query.single().1, joint));
                //     // }
                // }
                let point = mousepos.0;
                let filter = QueryFilter::only_fixed();

                rapier_context.intersections_with_point(point, filter, |entity| {
                    // Callback called on each collider with a shape containing the point.

                    let joint = RopeJointBuilder::new()
                        .local_anchor2(
                            mousepos.0 - ground_query.single().2.translation.truncate(), // Vec2::ZERO,
                        )
                        .limits([
                            0.5,
                            ground_query
                                .single()
                                .2
                                .translation
                                .distance(ball_query.single().0.translation),
                            // 500.0,
                        ])
                        .local_anchor1(ray_pos - ball_query.single().0.translation.truncate());

                    commands
                        .entity(entity)
                        .insert(ImpulseJoint::new(ball_query.single().1, joint));

                    println!("The entity {:?} contains the point.", entity);
                    // Return `false` instead if we want to stop searching for other colliders containing this point.
                    true
                });
            }
            ButtonState::Released => {
                commands
                    .entity(ground_query.single().0)
                    .remove::<ImpulseJoint>();
            }
        }
    }
}
