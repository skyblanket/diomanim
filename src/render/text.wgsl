// Text Rendering Shader
// Samples from texture atlas to render glyphs

struct TransformUniform {
    model_view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> transform: TransformUniform;

@group(1) @binding(0)
var atlas_texture: texture_2d<f32>;

@group(1) @binding(1)
var atlas_sampler: sampler;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = transform.model_view_proj * vec4<f32>(in.position, 1.0);
    out.uv = in.uv;
    out.color = in.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Sample the texture atlas
    let alpha = textureSample(atlas_texture, atlas_sampler, in.uv).a;

    // Multiply text color by glyph alpha
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
