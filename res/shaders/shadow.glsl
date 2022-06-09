#shader vertex
#version 460 core
#include "res/shaders/common/defs/light.glsl"

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec4 a_colour;
layout(location = 3) in vec2 a_texCoord;

layout(location = 4) in int a_materialIndex;
layout(location = 5) in mat4 a_transform;

layout (std430, binding = 0) buffer Lights {
    Light allLights[32];
    mat4 lightViews[32];
    mat4 lightProjection;
    uvec2 shadowMaps[32];
    int lightCount;
    vec3 cameraDir;
    vec3 cameraPos;
};

void main() {
    gl_Position = lightProjection * lightViews[0] * a_transform * vec4(a_position, 1.0);
}


///////////////////////////////////////////////////////////////////////////////////////
#shader fragment
#version 460 core

void main() {
}