struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct Circle {
    size: f32,
}

@group(1) @binding(0)
var t: texture_2d<f32>;

@group(1) @binding(1)
var s: sampler;

@group(3) @binding(0)
var<uniform> circle: Circle;

fn modulo_euclidean (a: f32, b: f32) -> f32 {
	var m = a % b;
	if (m < 0.0) {
		if (b < 0.0) {
			m -= b;
		} else {
			m += b;
		}
	}
	return m;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	let texture_size: vec2<f32> = vec2(f32(textureDimensions(t).x), f32(textureDimensions(t).y));
    let density = circle.size * 1.1;
	let new_res = vec2(
		texture_size.x / density,
		texture_size.y / density,
	);

	let scale = texture_size / new_res;
	let circle_pos = in.uv * new_res;
    let pos = vec2(
    modulo_euclidean(in.uv.x * texture_size.x,density),
    modulo_euclidean(in.uv.y * texture_size.y,density)) - vec2(density/2.0);
    let dist_squared = dot(pos, pos);
    var alpha = 1.0;
    if dist_squared > (circle.size * circle.size)/4.0 {
        alpha = 0.0;
    }

	let uv = (round(circle_pos - vec2(0.5)) * scale) / texture_size;
	// let uv = (round(circle_pos) * scale) / texture_size;
    return textureSample(t, s, uv) * in.color * alpha;
}
