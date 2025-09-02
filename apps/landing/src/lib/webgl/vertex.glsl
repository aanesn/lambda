attribute vec4 a_position;
attribute vec3 a_normal;
uniform mat4 u_matrix;
varying vec3 v_position;
varying vec3 v_normal;

void main() {
    gl_Position = u_matrix * a_position;
    v_position = a_position.xyz;
    v_normal = (u_matrix * vec4(a_normal, 0.0)).xyz;
}
