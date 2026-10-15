struct VertexInput {
    @location(0) position: vec2<f32>,

    @location(1) shape_position: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) colour: vec4<f32>,
    @location(4) corner_radius: f32,
    @location(5) border_width: f32,
    @location(6) border_colour: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) shape_position: vec2<f32>,
    @location(1) size: vec2<f32>,
    @location(2) colour: vec4<f32>,
    @location(3) corner_radius: f32,
    @location(4) border_width: f32,
    @location(5) border_colour: vec4<f32>,
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let pixel_position = model.shape_position + (model.position + vec2<f32>(0,-1.0)) * model.size;

    out.clip_position = vec4<f32>(pixel_position, 0, 1);

    out.shape_position = model.position * model.size;
    out.size = model.size;
    out.colour = model.colour;
    out.corner_radius = model.corner_radius;
    out.border_width = model.border_width;
    out.border_colour = model.border_colour;

    return out;
}

fn corner_sdf(position: vec2<f32>, size: vec2<f32>, radius: f32) -> f32 {
    let half_size = size / 2;

    let r = clamp(radius, 0.0, min(half_size.x, half_size.y));

    let q = abs(position - half_size) - (half_size - vec2<f32>(r));

    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - r;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let distance = corner_sdf(in.shape_position, in.size, in.corner_radius);

    let aa = max(fwidth(distance), 0.001);

    let coverage = 1.0 - smoothstep(-aa, aa, distance);

    let border_mask = select(
            0.0,
            smoothstep(-in.border_width - aa, -in.border_width + aa, distance),
            in.border_width > 0.0,
        );

    let colour = mix(in.colour, in.border_colour, border_mask);

    return vec4<f32>(colour.rgb, colour.a * coverage);
}
