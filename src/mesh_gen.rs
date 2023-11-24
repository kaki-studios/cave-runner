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
        app.add_systems(Startup, mesh_init)
            .add_systems(PostUpdate, mesh_update)
            .init_resource::<ColliderList>(); // PostUpdate because verts are added in main.rs
    }
}

#[derive(Component)]
struct ColliderMarker;

fn mesh_init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut mesh_empty = Mesh::new(PrimitiveTopology::TriangleList);
    mesh_empty.insert_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<Vec3>::new());
    //spawn an empty entity so we can modify it later in mesh_update
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(mesh_empty)),
            transform: Transform::default(),
            material: materials.add(ColorMaterial::from(Color::GRAY)),
            ..default()
        })
        .insert(MeshMarker);
}

fn mesh_update(
    verts: Res<VertsTest>,
    mut query: Query<&Mesh2dHandle, With<MeshMarker>>,
    mut meshes: ResMut<Assets<Mesh>>,
    timer: Res<VertTimer>,
    time: Res<Time>,
    mut commands: Commands,
    collider_query: Query<Entity, With<ColliderMarker>>,
    mut collider_list: ResMut<ColliderList>,
) {
    if timer.0.just_finished() {
        let mut count = 0;
        let _ = collider_query.iter().map(|_| count += 1);

        if collider_list.colliders.len() >= 48 {
            for _ in 0..=1 {
                println!("yo");
                commands.entity(collider_list.colliders[0]).despawn();
                collider_list.colliders.remove(0);
            }
        }

        // we get a handle to the mesh
        let handle = query.get_single_mut().expect("");
        //we get an optional mesh from the handle
        let mut mesh = meshes.get_mut(handle.0.id());
        if mesh.is_some() {
            //vertices holds all the vertices we want to apply to the mesh
            let mut vertices: Vec<Vec3> = vec![];
            for i in verts.verts.iter() {
                vertices.push(i.extend(0.0));
            }

            mesh.as_mut()
                .unwrap()
                .insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

            let mut indices: Vec<u16> = vec![];

            for i in 0..verts.verts.len() {
                //0 or 1
                if i % 4 < 2 {
                    if i + 6 < verts.verts.len() {
                        //first triangle
                        indices.push(i as u16);
                        indices.push((i + 6) as u16);
                        indices.push((i + 2) as u16);
                        //second triangle
                        indices.push(i as u16);
                        indices.push((i + 4) as u16);
                        indices.push((i + 6) as u16);
                    }
                }
            }

            mesh.unwrap()
                .set_indices(Some(Indices::U16(indices.clone())));

            if time.elapsed().as_secs_f32() > 1.5 {
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
                        .spawn(Collider::convex_polyline(vertices1).unwrap())
                        .insert(ColliderMarker)
                        .insert(RigidBody::Fixed)
                        .id();
                    let en2 = commands
                        .spawn(Collider::convex_polyline(vertices2).unwrap())
                        .insert(ColliderMarker)
                        .insert(RigidBody::Fixed)
                        .id();
                    collider_list.colliders.push(en1);
                    collider_list.colliders.push(en2);

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
