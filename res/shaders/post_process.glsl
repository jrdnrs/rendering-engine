#shader vertex
#version 450 core

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec3 a_tangent;
layout(location = 3) in vec4 a_colour;
layout(location = 4) in vec2 a_texCoord;

layout(location = 5) in uint a_materialIndex;
layout(location = 6) in mat4 a_transform;

out vec2 texCoord;

void main()
{
    gl_Position = vec4(a_position.x, a_position.y, 0.0, 1.0); 
    texCoord = a_texCoord;
}  


#shader fragment
#version 450 core
#extension GL_ARB_bindless_texture : require
#include "res/shaders/common/buffers/lightsBuffer.glsl"
#include "res/shaders/common/buffers/shadowMapsBuffer.glsl"

uniform sampler2D screenTexture;

in vec2 texCoord;

out vec4 FragColor;

void main()
{ 
    // float depthValue = texture( sampler2D(directionalShadowMap),texCoord).r;
    // FragColor = vec4(vec3(depthValue), 1.0);

    const float gamma = 2.2;
    const float exposure = 1.0;
    vec3 hdrColor = texture(screenTexture, texCoord).rgb;
  
    // exposure tone mapping
    vec3 mapped = vec3(1.0) - exp(-hdrColor * exposure);
    // gamma correction 
    mapped = pow(mapped, vec3(1.0 / gamma));
  
    FragColor = vec4(mapped, 1.0);
}