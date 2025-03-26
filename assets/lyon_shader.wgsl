#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> replace_color: vec4<f32>;
@group(2) @binding(1) var<uniform> clip_radius: f32;
@group(2) @binding(2) var<uniform> clip_center: vec2<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let col_dif = distance(mesh.color, replace_color);
    if (col_dif != 0) {
        return mesh.color;
    }

    let b = mesh.uv.x;
    let r = mesh.uv.y;
    var a = 1.0;

    // let dist = distance(mesh.uv, vec2<f32>(0.5));
    let dist = distance(mesh.uv, clip_center);
    if (dist > clip_radius) {
        discard;
    }

    // if (mesh.uv.y > size) {
    //     // a = 0.0;
    //     discard;
    // }
    let color = vec4<f32>(r, 1.0, b, a);
    return color;
}