#include "res/shaders/common/defs/material.glsl"

layout (std140, binding = 0) uniform Materials {
    Material materials[100];
};