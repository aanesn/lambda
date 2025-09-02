precision mediump float;
varying vec3 v_position;

void main() {
    float gradient = (v_position.y + 1.0) * 0.5;
    vec3 color = mix(vec3(0.8, 0.7, 0.9), vec3(0.9, 0.85, 0.95), gradient);
    gl_FragColor = vec4(color, 1.0);
}
