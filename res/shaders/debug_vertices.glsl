#shader vertex
#version 450 core
#include "res/shaders/common/buffers/matricesBuffer.glsl"


layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec3 a_tangent;
layout(location = 3) in vec4 a_colour;
layout(location = 4) in vec2 a_texCoord;

layout(location = 5) in uint a_materialIndex;
layout(location = 6) in mat4 a_transform;

out GEOMETRY {
    flat vec3 normal;
    flat vec3 tangent;
} geo_out;

void main() {
    geo_out.normal = a_normal;
    geo_out.tangent = a_tangent;

    // mat4 newView = mat4(mat3(projection * view));
    gl_Position = projection * view * a_transform * vec4(a_position, 1.0);
}

///////////////////////////////////////////////////////////////////////////////////////
#shader geometry
#version 450 core
#include "res/shaders/common/buffers/matricesBuffer.glsl"

layout (points) in;
layout (line_strip, max_vertices=4) out;

in GEOMETRY {
    flat vec3 normal;
    flat vec3 tangent;
} geo_in[];

out flat vec4 colour;

void main() {

    // normal
    colour = vec4(0.0, 1.0, 0.0, 1.0);
    gl_Position = gl_in[0].gl_Position;
    EmitVertex();
    gl_Position = gl_in[0].gl_Position + projection * view * vec4(geo_in[0].normal, 0.0) * 0.1;
    EmitVertex();
    EndPrimitive();

    // tangent
    colour = vec4(1.0, 0.0, 0.0, 1.0);
    gl_Position = gl_in[0].gl_Position;
    EmitVertex();
    gl_Position = gl_in[0].gl_Position + projection * view * vec4(geo_in[0].tangent, 0.0)  * 0.1;
    EmitVertex();
    EndPrimitive();

}


///////////////////////////////////////////////////////////////////////////////////////
#shader fragment
#version 450 core

in flat vec4 colour;

out vec4 FragColor;

void main() {
    FragColor = colour;
}