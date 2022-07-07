#shader vertex
#version 450 core
#include "res/shaders/common/buffers/lightsBuffer.glsl"
#include "res/shaders/common/buffers/generalBuffer.glsl"


layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec3 a_tangent;
layout(location = 3) in vec4 a_colour;
layout(location = 4) in vec2 a_texCoord;

layout(location = 5) in uint a_materialIndex;
layout(location = 6) in mat4 a_transform;


void main() {
    if (index1 == 0) {
        gl_Position = directionalLight.view * a_transform * vec4(a_position, 1.0);
    } else if (index1 < MAX_SPOT_LIGHTS + 1) {
        gl_Position = spotLights[index1].view * a_transform * vec4(a_position, 1.0);
    } else if (index1 < MAX_POINT_LIGHTS + MAX_SPOT_LIGHTS + 1) {
        
    }
}


///////////////////////////////////////////////////////////////////////////////////////
#shader fragment
#version 450 core

void main() {
}