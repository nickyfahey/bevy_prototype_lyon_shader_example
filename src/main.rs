use bevy::{
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::VertexAttributeValues,
        render_resource::{AsBindGroup, ShaderRef},
    },
    sprite::{Material2d, Material2dPlugin},
};
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::shapes::Circle;

/// This example uses a shader source file from the assets subdirectory
const SHADER_ASSET_PATH: &str = "lyon_shader.wgsl";

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CustomMaterial>::default(),
            ShapePlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(
            PostUpdate,
            set_uvs.after(bevy_prototype_lyon::plugin::BuildShapes),
        )
        .run();
}

// Setup a simple 2d scene
fn setup(mut commands: Commands, mut materials: ResMut<Assets<CustomMaterial>>) {
    commands.spawn(Camera2d);

    // Avoid using the ShapeBundle so we are not forced to include a ColorMaterial.
    commands.spawn((
        GeometryBuilder::build_as(&Circle {
            radius: 100.,
            center: Vec2::ZERO,
        }),
        Stroke::new(LinearRgba::GREEN, 50.),
        Fill::color(LinearRgba::BLUE),
        Mesh2d::default(),
        MeshMaterial2d(materials.add(CustomMaterial {
            replace_color: LinearRgba::GREEN,
        })),
    ));
}

/// Set the UVs for the mesh generated by `bevy_prototype_lyon` so they can be used by the texture.
fn set_uvs(mut meshes: ResMut<Assets<Mesh>>, mesh2ds: Query<&Mesh2d, Added<Mesh2d>>) {
    let mesh_handle = mesh2ds.get_single();

    if let Ok(mesh_handle) = mesh_handle {
        let mesh = meshes.get_mut(mesh_handle).unwrap();

        let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
        let VertexAttributeValues::Float32x3(positions) = positions else {
            panic!("Unexpected vertex format, expected Float32x3.");
        };

        let min: Vec2 = positions.iter().fold(Vec2::MAX, |acc, [x, y, _]| {
            Vec2::new(acc.x.min(*x), acc.y.min(*y))
        });

        let max: Vec2 = positions.iter().fold(Vec2::MIN, |acc, [x, y, _]| {
            Vec2::new(acc.x.max(*x), acc.y.max(*y))
        });

        let x_range = max.x - min.y;
        let y_range = max.y - min.y;

        // UVs go from [0,0] in the top left to [1,1] in the bottom right.
        let uvs: Vec<_> = positions
            .iter()
            .map(|[x, y, _]| {
                let uv_x = (x - min.x) / x_range;
                let uv_y = 1. - ((y - min.y) / y_range);
                [uv_x, uv_y]
            })
            .collect();

        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    }
}

// This is the struct that will be passed to the shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    /// This color will be replaced by the shader.
    #[uniform(0)]
    replace_color: LinearRgba,
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}
