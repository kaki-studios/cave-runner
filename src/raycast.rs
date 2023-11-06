use crate::mouseworldpos::MyWorldCoords;
use crate::{GroundMarker, MainCamera, PlayerMarker};
use bevy::input::mouse::MouseButtonInput;
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
        (Entity, Option<&mut ImpulseJoint>, &Transform),
        (
            With<GroundMarker>,
            Without<MainCamera>,
            Without<PlayerMarker>,
        ),
    >,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    asset_server: Res<AssetServer>,
) {
    //somehow raycast doesn't work perfectly after changing camera position to follow player...
    let ray_pos = Vec2::new(
        ball_query.single().0.translation.x,
        ball_query.single().0.translation.y + 50.5,
    );
    let ray_dir = mousepos.0;
    gizmos.line_2d(ray_pos, ray_dir, Color::LIME_GREEN);
    gizmos.circle_2d(mousepos.0, 5.0, Color::RED);

    use bevy::input::ButtonState;

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                let filter: QueryFilter = QueryFilter::new();
                // filter.exclude_collider(ball_query.single().1);

                let max_toi = 4_000_000.0;
                let solid = false;

                if let Some((entity, _toi)) =
                    rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter)
                {
                    if entity == ground_query.single().0 {
                        let hit_point = ray_pos + ray_dir * _toi;

                        // commands.spawn(SpriteBundle {
                        //     sprite: Sprite {
                        //         color: Color::rgb(0.25, 0.25, 0.75),
                        //         custom_size: Some(Vec2::new(10.0, 10.0)),
                        //         ..default()
                        //     },
                        //     transform: Transform::from_translation(Vec3::new(
                        //         hit_point.x,
                        //         hit_point.y,
                        //         0.0,
                        //     )),
                        //     texture: asset_server.load("square.png"),
                        //     ..default()
                        // });

                        let joint = RopeJointBuilder::new()
                            .local_anchor2(
                                // ground_query
                                //     .single()
                                //     .2
                                //     .transform_point(Vec3::new(-hit_point.x, hit_point.y, 0.))
                                //     .truncate(),
                                Vec2::ZERO,
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
                    }
                    println!(
                        "entity == ground query: {}",
                        entity == ground_query.single().0
                    );
                }
            }
            ButtonState::Released => {
                commands
                    .entity(ground_query.single().0)
                    .remove::<ImpulseJoint>();

                println!(
                    "MousePos: {}, BallPos {:?}",
                    mousepos.0,
                    ball_query.single().0.translation
                )
            }
        }
    }
}
