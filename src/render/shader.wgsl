struct Pos {
    [[builtin(position)]] position: vec4<f32>;
};

[[block]]
struct Transform {
    matrix: mat3x3<f32>;
};
[[group(0), binding(0)]]
var transform: Transform;

[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] idx: u32) -> Pos {
    var pos: vec3<f32> = vec3<f32>(0.0, 0.0, 1.0);

    if (idx > 0u && idx < 3u) {
        pos.y = 0.0;
    } else {
        pos.y = 10.0;
    }
    pos.x = (f32((idx+1u)%3u)) - 1.0;
    pos.x = pos.x * 3.0;

    pos = transform.matrix * pos;
    var out: Pos;
    out.position = vec4<f32>(pos.x, pos.y, 0.0, 1.0);

    return out;
}

[[stage(fragment)]]
fn fs_main() -> [[location(0)]] vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}