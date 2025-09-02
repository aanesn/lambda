attribute vec4 a_position;
uniform mat4 u_matrix;
varying vec3 v_position;

void main() {
    gl_Position = u_matrix * a_position;
    v_position = a_position.xyz;
}
