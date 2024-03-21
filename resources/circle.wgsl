struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
}

struct Circle {
    size: f32,
	res_x: f32,
	res_y: f32
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
	let uv = (round(circle_pos - vec2(0.5)) * scale) / texture_size;
	var color = textureSample(t, s, uv) * in.color;
    if dist_squared > (circle.size * circle.size)/4.0 {
        color.a = 0.0;
    }
	// let screen_distance = distance(in.position.xy, vec2(circle.res_x, circle.res_y));
	let screen_distance = distance(in.position.xy, vec2(circle.res_x / 2.0, circle.res_y / 2.0));

	if screen_distance > circle.res_y / 2.0 {
		color.a = 0.0;
	}

	if screen_distance < circle.res_y / 2.0 && color.a == 0.0 {
		color.r = 0.0;
		color.g = 0.0;
		color.b = 0.0;
		color.a = 1.0;
	}

	// let uv = (round(circle_pos) * scale) / texture_size;
    // return textureSample(t, s, uv) * in.color * alpha;
	return color;
// discard;
}
