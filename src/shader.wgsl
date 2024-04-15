struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(@location(0) position: vec4<f32>, @location(1) color: vec4<f32>) -> VertexOutput {
    var out: VertexOutput;
    // these values control the coordinates of the triangle
    out.position = position;
    out.color = color;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
