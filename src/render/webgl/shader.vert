#version 300 es

precision highp float;
precision highp int;

struct Pos {
    vec4 position;
};

uniform mat3x3 matrix;


void main() {
    uint idx = uint(gl_VertexID);
    vec3 pos = vec3(0.0, 0.0, 0.0);
    Pos out1;
    pos = vec3(0.0, 0.0, 1.0);
    if ((idx == 1u)) {
        pos.y = 1.0;
    } else {
        pos.y = 0.0;
    }
    pos.x = (float(idx) - 1.0);
    mat3x3 _e18 = matrix;
    vec3 _e19 = pos;
    pos = (_e18 * _e19);
    vec3 _e23 = pos;
    vec3 _e25 = pos;
    out1.position = vec4(_e23.x, _e25.y, 0.0, 1.0);
    Pos _e30 = out1;
    gl_Position = _e30.position;
    gl_Position.yz = vec2(-gl_Position.y, gl_Position.z * 2.0 - gl_Position.w);
    return;
}

