// Vertex Shader
struct VertexInput {
  @location(0) position: vec2<f32>,
  @location(1) color: vec3<f32>
};

struct VertexOutput {
  @builtin(position) Position: vec4<f32>,
  @location(0) color: vec3<f32>
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.Position = vec4<f32>(input.position, 0.0, 1.0);
    out.color = input.color;
    return out;
}

// Fragment Shader
@fragment
fn fs_main(output: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(output.color, 1.0);
}
