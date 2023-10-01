#version 450

layout(binding = 1) uniform sampler2D albedo_sampler;

layout(push_constant) uniform PushConstants {
    layout(offset = 64) float opacity;
} pcs;

layout(location = 0) in vec3 surface_color;
layout(location = 1) in vec2 surface_texel;

layout(location = 0) out vec4 output_color;

void main() {
    output_color = vec4(texture(albedo_sampler, surface_texel).rgb, pcs.opacity);
}
