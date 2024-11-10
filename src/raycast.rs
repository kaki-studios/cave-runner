use crate::mouseworldpos::MyWorldCoords;
use crate::{MainCamera, PlayerMarker};
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub struct RaycastPlugin;

impl Plugin for RaycastPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, grapple_hook)
            .init_resource::<GrapplePoint>();
    }
}
#[derive(Resource, Default)]
struct GrapplePoint {
    point: Vec2,
    should_draw: bool,
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
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    mut ext_forces: Query<&mut ExternalForce>,
    transforms: Query<(Entity, &Transform), (Without<PlayerMarker>, Without<MainCamera>)>,
    mut grapple_point: ResMut<GrapplePoint>,
) {
    for mut camera in &mut q_camera {
        camera.translation = ball_query.single().0.translation;
    }

    // let ray_dir = mousepos.0;
    // gizmos.line_2d(
    //     ball_query.single().0.translation.truncate(),
    //     ray_dir,
    //     Color::LIME_GREEN,
    // );

    if grapple_point.should_draw {
        gizmos.line_2d(
            ball_query.single().0.translation.truncate(),
            grapple_point.point,
            Color::WHITE,
        );
    }

    // gizmos.circle_2d(mousepos.0, 0.5, Color::RED);
    // gizmos.line_2d(
    //     ball_query.single().0.translation.truncate(),
    //     ball_query.single().0.translation.truncate() + ball_query.single().2.linvel,
    //     Color::RED,
    // );
    for ev in mousebtn_evr.read() {
        match ev.state {
            ButtonState::Pressed => {
                let point = mousepos.0;
                //only fixed so we dont raycast with the player
                let filter = QueryFilter::exclude_dynamic();

                rapier_context.intersections_with_point(point, filter, |entity| {
                    // Callback called on each collider with a shape containing the point.

                    //basically we just create a joint connecting the ground with the player.
                    let mut anchor: Vec2 = Vec2::ZERO;
                    for (en, tr) in transforms.iter() {
                        if en == entity {
                            anchor = tr.translation.truncate()
                        }
                    }
                    let joint = RopeJointBuilder::new(
                        Vec3::new(mousepos.0.x, mousepos.0.y, 0.0)
                            .distance(ball_query.single().0.translation),
                    )
                    .local_anchor1(mousepos.0 - anchor)
                    // .limits([
                    //     0.5,
                    //     Vec3::new(mousepos.0.x, mousepos.0.y, 0.0)
                    //         .distance(ball_query.single().0.translation)
                    //         / 1.33,
                    //     // 500.0,
                    // ])
                    .local_anchor2(Vec2::ZERO);

                    commands
                        .entity(ball_query.single().1)
                        .insert(ImpulseJoint::new(entity, joint));
                    grapple_point.point = mousepos.0;
                    grapple_point.should_draw = true;
                    //we add a force

                    //GravityScale to 0 for better mechanics
                    *ball_query.single_mut().3 = GravityScale(0.0);

                    for mut ext_force in ext_forces.iter_mut() {
                        ext_force.force =
                            (-ball_query.single().0.translation.truncate() + mousepos.0) * 10000.0;

                        ext_force.torque = 1000.0;
                    }

                    false
                    // Return `false` if we want to stop searching for other colliders containing this point.
                });
            }
            ButtonState::Released => {
                commands
                    .entity(ball_query.single().1)
                    .remove::<ImpulseJoint>();

                for mut ext_force in ext_forces.iter_mut() {
                    ext_force.force = Vec2::ZERO;
                    ext_force.torque = 0.0;
                }
                grapple_point.should_draw = false;

                *ball_query.single_mut().3 = GravityScale(1.0);
            }
        }
    }
}
