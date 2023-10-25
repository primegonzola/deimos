#version 450

// bindings
layout(binding = 0) uniform CameraUniform {
    mat4 model;
    mat4 view;
    mat4 projection;
} camera;

// inputs
layout(location = 0) in vec3 aPosition;
layout(location = 1) in vec2 aTexel;

// outputs
layout(location = 0) out vec2 vTexel;

void main() {
    gl_Position = camera.projection * camera.view * camera.model * vec4(aPosition, 1.0);
    vTexel = aTexel;
}
