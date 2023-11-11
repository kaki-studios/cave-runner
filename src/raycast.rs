use crate::mouseworldpos::MyWorldCoords;
use crate::{GroundMarker, MainCamera, PlayerMarker};
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub struct RaycastPlugin;

impl Plugin for RaycastPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, grapple_hook);
    }
}

fn grapple_hook(
    rapier_context: Res<RapierContext>,

    mousepos: Res<MyWorldCoords>,

    mut q_camera: Query<&mut Transform, With<MainCamera>>,
    mut gizmos: Gizmos,
    mut ball_query: Query<
        (&Transform, Entity, &mut Velocity, &mut GravityScale),
        (With<PlayerMarker>, Without<MainCamera>),
    >,
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
    mut ext_forces: Query<&mut ExternalForce>,
) {
    for mut camera in &mut q_camera {
        camera.translation = ball_query.single().0.translation;
    }

    //somehow raycast doesn't work perfectly after changing camera position to follow player...
    let ray_dir = mousepos.0;
    gizmos.line_2d(
        ball_query.single().0.translation.truncate(),
        ray_dir,
        Color::LIME_GREEN,
    );
    gizmos.circle_2d(mousepos.0, 0.5, Color::RED);
    gizmos.line_2d(
        ball_query.single().0.translation.truncate(),
        ball_query.single().0.translation.truncate() + ball_query.single().2.linvel,
        Color::RED,
    );
    for ev in mousebtn_evr.read() {
        match ev.state {
            ButtonState::Pressed => {
                let point = mousepos.0;
                //only fixed so we dont raycast with the player
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
                                Vec3::new(mousepos.0.x, mousepos.0.y, 0.0)
                                    .distance(ball_query.single().0.translation),
                                // 500.0,
                            ])
                            .local_anchor1(Vec2::ZERO);

                        commands
                            .entity(entity)
                            .insert(ImpulseJoint::new(ball_query.single().1, joint));

                        //we add a force
                        let mut velocity_vec = Vec2::ZERO;
                        for (_, _, rb_vels, _) in &mut ball_query {
                            velocity_vec = rb_vels.linvel;
                        }
                        *ball_query.single_mut().3 = GravityScale(0.05);

                        for mut ext_force in ext_forces.iter_mut() {
                            ext_force.force = velocity_vec * 1000.0;
                            ext_force.torque = 100.0;
                        }

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

                for mut ext_force in ext_forces.iter_mut() {
                    ext_force.force = Vec2::ZERO;
                    ext_force.torque = 0.0;
                }

                *ball_query.single_mut().3 = GravityScale(1.0);
            }
        }
    }
}
