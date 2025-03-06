#version 300 es
precision highp float;

in vec2 vUv;
in vec3 vNormal;
in vec3 vViewPosition;

uniform sampler2D diffuseMap;
uniform sampler2D normalMap;
uniform sampler2D emissiveMap;
uniform float time;
uniform vec3 color;
uniform float level;

out vec4 fragColor;

// Noise function for fire effect
float noise(vec3 p) {
    return fract(sin(dot(p, vec3(12.9898, 78.233, 45.164))) * 43758.5453);
}

void main() {
    // Sample textures
    vec4 diffuse = texture(diffuseMap, vUv);
    vec3 normal = normalize(texture(normalMap, vUv).rgb * 2.0 - 1.0);
    vec3 emissive = texture(emissiveMap, vUv + vec2(time * 0.1)).rgb;

    // Fire effect
    float noise1 = noise(vec3(vUv * 10.0, time));
    float noise2 = noise(vec3(vUv * 20.0, time * 2.0));
    vec3 fireColor = mix(color, vec3(1.0), noise1 * noise2);

    // Level-based effects
    float intensity = 1.0 + float(level > 15.0) * (noise1 * 0.5);
    vec3 glow = emissive * fireColor * intensity;

    fragColor = vec4(diffuse.rgb * fireColor + glow, diffuse.a);
}
