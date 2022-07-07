#shader vertex
#version 450 core
#include "res/shaders/common/buffers/matricesBuffer.glsl"
#include "res/shaders/common/buffers/materialsBuffer.glsl"


layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec3 a_tangent;
layout(location = 3) in vec4 a_colour;
layout(location = 4) in vec2 a_texCoord;

layout(location = 5) in uint a_materialIndex;
layout(location = 6) in mat4 a_transform;

out VS_OUT {
    vec3 texCoord;
    flat Material material;
} vs_out;


void main() {
    vs_out.texCoord = a_position;
    vs_out.material = materials[a_materialIndex];

    mat4 newView = mat4(mat3(view));

    // vector is divided by w (which is z, prior to any transforms)
    // but we need this to be at max distance, so we pass w as z
    // so that w/w = 1
    vec4 pos = projection * newView * vec4(a_position, 1.0);
    gl_Position = pos.xyww;
}


///////////////////////////////////////////////////////////////////////////////////////
#shader fragment
#version 450 core
#extension GL_ARB_bindless_texture : require
#include "res/shaders/common/defs/material.glsl"

in VS_OUT {
    vec3 texCoord;
    flat Material material;
} vs_in;

out vec4 FragColor;

void main() {
    FragColor = texture(samplerCube(vs_in.material.diffuseTexture), vs_in.texCoord);
}