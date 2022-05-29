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
    vec3 normal;
    Material material;
} vs_out;

layout (std430, binding = 0) buffer Vertex {
    Material materials[100];
    mat4 transforms[1000];
    mat4 projection;
    mat4 view;
    uint materialIndex[1000]; // this uses 16 bytes per instead of just 4
};


void main() {
    mat4 transform = transforms[a_index];

    vs_out.normal = a_normal;
    vs_out.texCoord = a_texCoord;
    vs_out.material = materials[materialIndex[a_index]];

    gl_Position = projection * view * transform * vec4(a_position, 1.0);
}

///////////////////////////////////////////////////////////////////////////////////////
#shader geometry
#version 460 core
#include "res/shaders/common/materialDef.glsl"

layout(triangles) in;
layout(triangle_strip, max_vertices = 3) out;

in VS_OUT {
    vec2 texCoord;
    vec3 normal;
    Material material;
} vs_in[];


out GS_OUT {
    vec2 texCoord;
    Material material;
} gs_out;


void main() {
    for (int i = 0; i < gl_in.length(); i++) {
        gs_out.texCoord = vs_in[i].texCoord;
        gs_out.material = vs_in[i].material;
        gl_Position = gl_in[i].gl_Position;
        EmitVertex();
    }

    EndPrimitive();
}


///////////////////////////////////////////////////////////////////////////////////////
#shader fragment
#version 460 core
#extension GL_ARB_bindless_texture : require
#include "res/shaders/common/materialDef.glsl"

in GS_OUT {
    vec2 texCoord;
    Material material;
} gs_in;

out vec4 FragColour;

void main() {
    FragColour = texture(sampler2D(gs_in.material.diffuseTexture), gs_in.texCoord);
}