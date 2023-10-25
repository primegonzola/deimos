#version 450

// bindings
layout(binding = 1) uniform sampler2D albedo_sampler;

// inputs
layout(location = 0) in vec2 vTexel;

// outputs
layout(location = 0) out vec4 surface_color;

void main() {
    surface_color = texture(albedo_sampler, vTexel);
}
