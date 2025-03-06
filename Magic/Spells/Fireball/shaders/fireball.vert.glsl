#version 300 es
precision highp float;

in vec3 position;
in vec3 normal;
in vec2 uv;
in vec3 instancePosition;
in vec4 instanceRotation;
in float instanceScale;

uniform mat4 projectionMatrix;
uniform mat4 viewMatrix;
uniform float time;

out vec2 vUv;
out vec3 vNormal;
out vec3 vViewPosition;

void main() {
    vUv = uv;

    // Apply instance transformation
    vec3 pos = position * instanceScale;
    vec3 n = normal;

    // Quaternion rotation
    vec4 q = instanceRotation;
    vec3 rotated = pos + 2.0 * cross(q.xyz, cross(q.xyz, pos) + q.w * pos);
    vec3 rotatedNormal = n + 2.0 * cross(q.xyz, cross(q.xyz, n) + q.w * n);

    // Final position
    vec4 worldPosition = vec4(rotated + instancePosition, 1.0);
    vec4 viewPosition = viewMatrix * worldPosition;

    vNormal = normalize(mat3(viewMatrix) * rotatedNormal);
    vViewPosition = -viewPosition.xyz;

    gl_Position = projectionMatrix * viewPosition;
}
