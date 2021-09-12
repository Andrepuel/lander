precision highp float;
precision highp int;

attribute vec2 a_position;
uniform mat3 matrix;

void main() {
    vec3 pos = vec3(a_position.x, a_position.y, 1.0);
    pos = (matrix * pos);
    gl_Position = vec4(pos.x, -pos.y, 0.0, 1.0);
}

