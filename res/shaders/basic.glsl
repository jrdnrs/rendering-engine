#shader vertex
#version 460 core
#include "res/shaders/common/materialDef.glsl"

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec4 a_colour;
layout(location = 3) in vec2 a_texCoord;
layout(location = 4) in int a_index;


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

struct InstanceData {
    mat4 transform;
    uint materialIndex;
};

layout (std430, binding = 4) buffer InstanceDatas {
    InstanceData instanceData[1000];
};


void main() {
    InstanceData thisInstance = instanceData[a_index];

    mat4 transform = thisInstance.transform;

    vs_out.texCoord = a_texCoord;
    vs_out.material = materials[thisInstance.materialIndex];

    gl_Position = projection * view * transform * vec4(a_position, 1.0);
}


///////////////////////////////////////////////////////////////////////////////////////
#shader fragment
#version 460 core
#extension GL_ARB_bindless_texture : require
#include "res/shaders/common/materialDef.glsl"

in VS_OUT {
    vec2 texCoord;
    Material material;
} vs_in;

out vec4 FragColour;

void main() {
    // FragColour = vec4(0.5, 0.2, 0.8, 1.0);
    FragColour = texture(sampler2D(vs_in.material.diffuseTexture), vs_in.texCoord);
}