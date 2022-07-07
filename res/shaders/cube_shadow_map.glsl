#shader vertex
#version 450 core

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec3 a_tangent;
layout(location = 3) in vec4 a_colour;
layout(location = 4) in vec2 a_texCoord;

layout(location = 5) in uint a_materialIndex;
layout(location = 6) in mat4 a_transform;


void main() {
    gl_Position = a_transform * vec4(a_position, 1.0);
}


///////////////////////////////////////////////////////////////////////////////////////
#shader geometry
#version 450 core
#include "res/shaders/common/buffers/lightsBuffer.glsl"
#include "res/shaders/common/buffers/generalBuffer.glsl"

layout (triangles) in;
layout (triangle_strip, max_vertices=18) out;

void main() {
      for (int face = 0; face < 6; ++face) {
        gl_Layer = face; // built-in variable that specifies to which face we render.
        for (int i = 0; i < 3; ++i) {
            gl_Position = pointLights[index1].views[face] * gl_in[i].gl_Position;
            EmitVertex();
        }    
        EndPrimitive();
    }
}


///////////////////////////////////////////////////////////////////////////////////////
#shader fragment
#version 450 core

void main() {
}