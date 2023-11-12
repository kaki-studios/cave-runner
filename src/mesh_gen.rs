use crate::VertsTest;
use bevy::prelude::*;
use bevy::render::mesh::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

#[derive(Component)]
struct MeshMarker;

pub struct MeshGenPlugin;

impl Plugin for MeshGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, mesh_init);
        // .add_systems(Update, mesh_update); //we don't add the system because it panics at line
        // 52 because the mesh we got doesn't have any ATTRIBUTE_POSITION
    }
}

fn mesh_init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh_empty = Mesh::new(PrimitiveTopology::TriangleList);
    println!("plugin works");

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
) {
    //we get a handle to the mesh
    let handle = query.get_single_mut().expect("");
    //we get a mesh from the handle
    let mesh = meshes.get_mut(handle.0.id()); // Error caused here
    if mesh.is_some() {
        //positions holds all the vertices, which might be none
        let positions = mesh
            .as_ref()
            .expect("shit")
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .unwrap();
        //now thing holds the vertices, making sure they exist
        if let VertexAttributeValues::Float32x2(thing) = positions {
            //the vertices vector is out new vertices
            let mut vertices: Vec<Vec2> = Vec::new();
            //for each old vertex...
            for i in thing {
                //we create temp describing the position of a single vertex
                let temp = Vec2::new(i[0], i[1]);
                // Modify temp here
                vertices.push(temp);
            }

            mesh.unwrap()
                .insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        }
    }
}
