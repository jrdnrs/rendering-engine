#shader vertex
#version 460 core

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec4 a_colour;
layout(location = 3) in vec2 a_texCoord;
layout(location = 4) in int a_index;

out VS_OUT {
    vec4 colour;
} vs_out;

layout (std430, binding = 3) buffer Matrices {
    mat4 projection;
    mat4 view;
};


void main() {
    vs_out.colour = a_colour;

    mat4 newView = mat4(mat3(view));
    gl_Position = newView * vec4(a_position, 1.0);
}


///////////////////////////////////////////////////////////////////////////////////////
#shader fragment
#version 460 core

in VS_OUT {
    vec4 colour;
} vs_in;

out vec4 FragColour;

void main() {
    FragColour = vs_in.colour;
}