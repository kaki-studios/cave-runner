use crate::VertTimer;
use crate::VertsTest;
use bevy::prelude::*;
use bevy::render::mesh::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

#[derive(Component)]
struct MeshMarker;

pub struct MeshGenPlugin;

impl Plugin for MeshGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, mesh_init)
            .add_systems(Update, mesh_update);
    }
}

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
) {
    if timer.0.just_finished() {
        //we get a handle to the mesh
        let handle = query.get_single_mut().expect("");
        //we get an optional mesh from the handle
        let mut mesh = meshes.get_mut(handle.0.id());
        if mesh.is_some() {
            //vertices holds all the vertices we want to apply to the mesh
            let mut vertices: Vec<Vec3> = vec![];

            for i in 0..verts.verts.len() {
                vertices.push(verts.verts[i].extend(0.0));
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

            mesh.unwrap().set_indices(Some(Indices::U16(indices)));
        }
    }
}
