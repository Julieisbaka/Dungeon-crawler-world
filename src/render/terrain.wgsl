struct Uniforms {
    view_projection: mat4x4<f32>,
    camera_position: vec4<f32>,
    light_direction: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec3<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.clip_position = uniforms.view_projection * vec4<f32>(input.position, 1.0);
    output.world_position = input.position;
    output.normal = normalize(input.normal);
    output.color = input.color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let light_dir = normalize(uniforms.light_direction.xyz);
    let view_dir = normalize(uniforms.camera_position.xyz - input.world_position);
    let normal = normalize(input.normal);
    let diffuse = max(dot(normal, light_dir), 0.0);
    let rim = pow(1.0 - max(dot(normal, view_dir), 0.0), 2.0) * 0.18;
    let distance = length(uniforms.camera_position.xyz - input.world_position);
    let fog = clamp(distance / 260.0, 0.0, 0.72);
    let lit = input.color * (0.22 + diffuse * 0.82 + rim);
    let fog_color = vec3<f32>(0.015, 0.018, 0.02);
    return vec4<f32>(mix(lit, fog_color, fog), 1.0);
}
