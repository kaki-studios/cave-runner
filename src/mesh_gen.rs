use crate::VertTimer;
use crate::VertsTest;
use bevy::prelude::*;
use bevy::render::mesh::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct MeshMarker;

#[derive(Resource, Default)]
struct ColliderList {
    colliders: Vec<Entity>,
}

pub struct MeshGenPlugin;

impl Plugin for MeshGenPlugin {
    fn build(&self, app: &mut App) {
        app //.add_systems(Startup, mesh_init)
            .add_systems(PostUpdate, mesh_update)
            .init_resource::<ColliderList>(); // PostUpdate because verts are added in main.rs
    }
}

#[derive(Component)]
struct ColliderMarker;

// fn mesh_init(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     let mut mesh_empty = Mesh::new(PrimitiveTopology::TriangleList);
//     mesh_empty.insert_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<Vec3>::new());
//     //spawn an empty entity so we can modify it later in mesh_update
//     commands
//         .spawn(MaterialMesh2dBundle {
//             mesh: Mesh2dHandle(meshes.add(mesh_empty)),
//             transform: Transform::default(),
//             material: materials.add(ColorMaterial::from(Color::GRAY)),
//             // visibility: Visibility::Visible,
//             ..default()
//         })
//         .insert(MeshMarker);
// }

fn mesh_update(
    verts: Res<VertsTest>,
    mut meshes: ResMut<Assets<Mesh>>,
    timer: Res<VertTimer>,
    mut commands: Commands,
    mut collider_list: ResMut<ColliderList>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    //every 0.5 seconds
    if timer.0.just_finished() {
        if collider_list.colliders.len() >= 100 {
            for _ in 0..4 {
                commands.entity(collider_list.colliders[0]).despawn();
                collider_list.colliders.remove(0);
            }
        }

        if verts.verts.len() >= 8 {
            //it crashes otherwise
            for chunk in verts.verts.windows(8).rev().take(1) {
                //we make 2 colliders per "chunk"
                let mut vertices1: Vec<Vec2> = vec![];
                vertices1.push(chunk[4]);
                vertices1.push(chunk[6]);
                vertices1.push(chunk[2]);
                vertices1.push(chunk[0]);

                let mut vertices2: Vec<Vec2> = vec![];

                vertices2.push(chunk[7]);
                vertices2.push(chunk[5]);
                vertices2.push(chunk[1]);
                vertices2.push(chunk[3]);

                let en1 = commands
                    .spawn(Collider::convex_polyline(vertices1.clone()).unwrap())
                    // .insert(ColliderMarker)
                    .insert(RigidBody::Fixed)
                    .id();
                let en2 = commands
                    .spawn(Collider::convex_polyline(vertices2.clone()).unwrap())
                    // .insert(ColliderMarker)
                    .insert(RigidBody::Fixed)
                    .id();
                collider_list.colliders.push(en1);
                collider_list.colliders.push(en2);

                let indices: Vec<u16> = vec![0, 1, 2, 2, 3, 0];

                let vertices13: Vec<Vec3> =
                    vertices1.into_iter().map(|vert| vert.extend(0.0)).collect();
                let vertices23: Vec<Vec3> =
                    vertices2.into_iter().map(|vert| vert.extend(0.0)).collect();

                let mut mesh1 = Mesh::new(PrimitiveTopology::TriangleList);
                mesh1.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices13);
                mesh1.set_indices(Some(Indices::U16(indices.clone())));

                let mesh11 = commands
                    .spawn(MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(mesh1)),
                        transform: Transform::default(),
                        material: materials.add(ColorMaterial::from(Color::GRAY)),
                        // visibility: Visibility::Visible,
                        ..default()
                    })
                    .id();

                let mut mesh2 = Mesh::new(PrimitiveTopology::TriangleList);
                mesh2.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices23);
                mesh2.set_indices(Some(Indices::U16(indices)));

                let mesh22 = commands
                    .spawn(MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(mesh2)),
                        transform: Transform::default(),
                        material: materials.add(ColorMaterial::from(Color::GRAY)),
                        // visibility: Visibility::Visible,
                        ..default()
                    })
                    .id();
                collider_list.colliders.push(mesh11);
                collider_list.colliders.push(mesh22);

                // println!("spawned!");
            }
            // commands
            //     .spawn(Collider::convex_decomposition(
            //         &verts.verts,
            //         &vec2slice(indices)[..],
            //     ))
            //     .insert(ColliderMarker);
        }
    }
}

// fn vec2slice(indices: Vec<u16>) -> Vec<[u32; 2]> {
//     let mut indices_new: Vec<[u32; 2]> = vec![];
//     for index in indices.windows(2) {
//         // println!("{}", index[0]);
//
//         let new: [u32; 2] = [index[0] as u32, index[1] as u32];
//         indices_new.push(new);
//     }
//     indices_new
// }
