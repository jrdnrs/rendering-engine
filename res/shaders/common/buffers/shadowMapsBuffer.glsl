#include "res/shaders/common/buffers/lightsBuffer.glsl"

layout (std140, binding = 1) uniform ShadowMaps {
    uvec2 pointShadowMaps[MAX_POINT_LIGHTS];
    uvec2 spotShadowMaps[MAX_SPOT_LIGHTS];
    uvec2 directionalShadowMap;
};