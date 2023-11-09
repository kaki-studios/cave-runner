use crate::mouseworldpos::MyWorldCoords;
use crate::{GroundMarker, MainCamera, PlayerMarker};
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub struct RaycastPlugin;

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
    let ray_dir = mousepos.0;
    gizmos.line_2d(
        ball_query.single().0.translation.truncate(),
        ray_dir,
        Color::LIME_GREEN,
    );
    gizmos.circle_2d(mousepos.0, 5.0, Color::RED);

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                let point = mousepos.0;
                //only fixed so we dont pointcast with the player
                let filter = QueryFilter::only_fixed();

                rapier_context.intersections_with_point(point, filter, |entity| {
                    // Callback called on each collider with a shape containing the point.

                    //basically we just create a joint connecting the ground with the player. doesn't support multiple
                    //ground objects

                    if entity == ground_query.single().0 {
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
                            .local_anchor1(Vec2::ZERO);

                        commands
                            .entity(entity)
                            .insert(ImpulseJoint::new(ball_query.single().1, joint));

                        println!("The entity {:?} contains the point.", entity);
                        return false;
                        // Return `false` if we want to stop searching for other colliders containing this point.
                    }
                    //we didn't hit the correct entity, so we keep searching for other colliders by
                    //returning true
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
