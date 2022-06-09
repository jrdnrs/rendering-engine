#shader vertex
#version 460 core

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec4 a_colour;
layout(location = 3) in vec2 a_texCoord;

layout(location = 4) in int a_materialIndex;
layout(location = 5) in mat4 a_transform;

out vec2 texCoord;

void main()
{
    gl_Position = vec4(a_position.x, a_position.y, 0.0, 1.0); 
    texCoord = a_texCoord;
}  


#shader fragment
#version 460 core
#extension GL_ARB_bindless_texture : require

out vec4 FragColor;
  
in vec2 texCoord;

uniform sampler2D screenTexture;

#include "res/shaders/common/defs/light.glsl"
layout (std430, binding = 0) buffer Lights {
    Light allLights[32];
    mat4 lightViews[32];
    mat4 lightProjection;
    uvec2 shadowMaps[32];
    int lightCount;
    vec3 cameraDir;
    vec3 cameraPos;
};

void main()
{ 
    // float depthValue = texture(sampler2D(shadowMaps[0]), texCoord).r;
    // FragColor = vec4(vec3(depthValue), 1.0);

    float gamma = 2.2;
    FragColor = vec4(pow(texture(screenTexture, texCoord).rgb, vec3(1.0/gamma)), 1.0);
}