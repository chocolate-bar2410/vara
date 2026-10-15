struct VertexInput {
    @location(0) position: vec2<f32>,

    @location(1) shape_position: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) colour: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) shape_position: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) colour: vec4<f32>,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let pixel_position = model.shape_position + model.position * model.size;

    out.clip_position = vec4<f32>(pixel_position, 0, 1);

    out.shape_position = model.position * model.size;
    out.size = model.size;
    out.colour = model.colour;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.colour.rgb, in.colour.a);
}
