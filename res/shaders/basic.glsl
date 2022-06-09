#shader vertex
#version 460 core
#include "res/shaders/common/defs/material.glsl"

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec4 a_colour;
layout(location = 3) in vec2 a_texCoord;

layout(location = 4) in int a_materialIndex;
layout(location = 5) in mat4 a_transform;


out VS_OUT {
    vec2 texCoord;
    Material material;
} vs_out;

layout (std430, binding = 2) buffer Materials {
    Material materials[100];
};

layout (std430, binding = 3) buffer Matrices {
    mat4 projection;
    mat4 view;
};



void main() {
    vs_out.texCoord = a_texCoord;
    vs_out.material = materials[a_materialIndex];

    gl_Position = projection * view * a_transform * vec4(a_position, 1.0);
}


///////////////////////////////////////////////////////////////////////////////////////
#shader fragment
#version 460 core
#extension GL_ARB_bindless_texture : require
#include "res/shaders/common/defs/material.glsl"

in VS_OUT {
    vec2 texCoord;
    Material material;
} vs_in;

out vec4 FragColour;

void main() {
    FragColour = texture(sampler2D(vs_in.material.diffuseTexture), vs_in.texCoord);
}