precision mediump float;
varying vec3 v_position;
varying vec3 v_normal;

void main() {
    vec3 normal = normalize(v_normal);
    vec3 viewDir = normalize(-v_position);
    float fresnel = 1.0 - abs(dot(normal, viewDir));
    
    vec3 baseColor = vec3(0.6, 0.5, 0.7);
    vec3 edgeColor = vec3(0.8, 0.8, 0.8);
    vec3 finalColor = mix(baseColor, edgeColor, pow(fresnel, 6.0));

    gl_FragColor = vec4(finalColor, 1.0);
}
