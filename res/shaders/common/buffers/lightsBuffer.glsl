#include "res/shaders/common/defs/lights.glsl"

#define MAX_POINT_LIGHTS 2
#define MAX_SPOT_LIGHTS 4

layout (std140, binding = 2) uniform Lights {
    PointLight pointLights[MAX_POINT_LIGHTS];
    SpotLight spotLights[MAX_SPOT_LIGHTS];
    DirectionalLight directionalLight;

    int pointLightCount;
    int spotLightCount;
    int directionalLightCount;
    
    vec3 cameraDir;
    vec3 cameraPos;
};