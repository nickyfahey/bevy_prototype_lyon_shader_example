#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> replace_color: vec4<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let col_dif = distance(mesh.color, replace_color);
    if (col_dif != 0) {
        return mesh.color;
    }

    return vec4<f32>(mesh.uv.x, 1.0, mesh.uv.y, 1.0);
}