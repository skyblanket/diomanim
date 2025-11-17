// Uniform buffer for transform matrices
struct Uniforms {
    model_view_proj: mat4x4<f32>,
    color: vec4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
