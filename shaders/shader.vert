#version 450

layout(binding = 0) uniform UniformBufferObject {
    mat4 view;
    mat4 proj;
} ubo;

layout(push_constant) uniform PushConstants {
    mat4 model;
} pcs;

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 texel;
layout(location = 2) in vec3 color;

layout(location = 0) out vec3 surface_color;
layout(location = 1) out vec2 surface_texel;

void main() {
    gl_Position = ubo.proj * ubo.view * pcs.model * vec4(position, 1.0);
    surface_color = color;
    surface_texel = texel;
}
