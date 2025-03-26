# bevy_prototype_lyon + Custom Shader

This is a simple example project that shows how a custom shader can be used alongside [bevy_prototype_lyon](https://github.com/Nilirad/bevy_prototype_lyon).

Works with `bevy_prototype_lyon` v0.13.0 but at time of writing changes in the master brach would force `ColorMaterial` to be used.

This is cobbled together from various examples scattered about the web, with thanks to the writers of the following:
- [webgpufundamentals: WGSL](https://webgpufundamentals.org/webgpu/lessons/webgpu-wgsl.html)
- [Bevy Examples: Shader Material](https://bevyengine.org/examples/shaders/shader-material-2d/)
- [Bevy Examples: Generate Custom Mesh](https://bevyengine.org/examples/3d-rendering/generate-custom-mesh/)
- [How to render a shader outline for a 2d mesh?](https://github.com/bevyengine/bevy/discussions/7102) - Never got an answer but the code given was helpful in working out how to set uvs.
- [Bevy's color material shader](https://github.com/bevyengine/bevy/blob/release-0.15.3/crates/bevy_sprite/src/mesh2d/color_material.wgsl)
