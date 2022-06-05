#shader vertex
#version 460 core

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec4 a_colour;
layout(location = 3) in vec2 a_texCoord;
layout(location = 4) in int a_index;

out vec2 texCoord;

void main()
{
    gl_Position = vec4(a_position.x, a_position.y, 0.0, 1.0); 
    texCoord = a_texCoord;
}  


#shader fragment
#version 460 core

out vec4 FragColor;
  
in vec2 texCoord;

uniform sampler2D screenTexture;

void main()
{ 
    float gamma = 2.2;
    FragColor = vec4(pow(texture(screenTexture, texCoord).rgb, vec3(1.0/gamma)), 1.0);
}