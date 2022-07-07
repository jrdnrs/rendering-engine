#include "res/shaders/common/comparison.glsl"
#include "res/shaders/common/util.glsl"

#line 0 10

#define PI  3.141592653
#define TAU 6.283185307

// http://maxest.gct-game.net/content/chss.pdf
float penumbraWidth(sampler2D shadowMap, vec2 shadowMapUV, vec2 texelSize, float currentDepth, float diskRotation, int sampleCount) {
    const float maxPenumbraFilterSize = 100.0;

    float blockersDepth = 0.0;
    float blockersCount = 0.0;

    for (int i = 0; i < sampleCount; i++) {
        vec2 sampleUV = shadowMapUV + VogelDisk(i, sampleCount, diskRotation) * texelSize * maxPenumbraFilterSize;
        float shadowMapDepth = texture(shadowMap, sampleUV).r;

        if (shadowMapDepth < currentDepth) {
            blockersDepth += shadowMapDepth;
            blockersCount++;
        }
    }

    if (blockersCount > 0.0) {
        blockersDepth /= blockersCount;
        // TODO: make this variable via uniform
        float lightSize = 40.0;

        return lightSize * (currentDepth - blockersDepth) / blockersDepth;
    }   

    return 0.0;
}

float shadowWithPenumbra(sampler2D shadowMap, vec3 lightToPos_ls) {
    const int SAMPLES = 8;

    // transform to [0,1] range
    vec3 projCoords = clamp(lightToPos_ls * 0.5 + 0.5, 0.0, 1.0);
    vec2 shadowMapUV = projCoords.xy;
    float currentDepth = projCoords.z - 0.0001;
    vec2 texelSize = 1.0 / textureSize(shadowMap, 0);

    float rotation = InterleavedGradientNoise(gl_FragCoord.xy) * TAU;
    float penumbraWidth = penumbraWidth(shadowMap, shadowMapUV, texelSize, currentDepth, rotation, SAMPLES);

    float shadow = 0.0;

    for (int i = 0; i < SAMPLES; i++) {
        vec2 sampleUV = shadowMapUV + (penumbraWidth * VogelDisk(i, SAMPLES, rotation) * texelSize);
        float shadowMapDepth = texture(shadowMap, sampleUV).r;
        shadow += step(shadowMapDepth, currentDepth);
    }

    shadow /= SAMPLES;

    return shadow;
}

float hardShadow(sampler2D shadowMap, vec3 lightToPos_ls) {
    // transform to [0,1] range
    vec3 projCoords = clamp(lightToPos_ls * 0.5 + 0.5, 0.0, 1.0);
    vec2 shadowMapUV = projCoords.xy;
    float currentDepth = projCoords.z - 0.0001;

    float shadowMapDepth = texture(shadowMap, shadowMapUV).r;
    return step(shadowMapDepth, currentDepth);
}

float calcDirShadow(sampler2D shadowMap, vec3 lightToPos_ls, float lightCos) {
    return shadowWithPenumbra(shadowMap, lightToPos_ls);
}  


float calcOmniShadow(samplerCubeShadow shadowMap, vec3 lightToPos_ws, float lightToPosDepth_ws) {
    float shadow = texture(shadowMap, vec4(lightToPos_ws, lightToPosDepth_ws)).r;

    return shadow * (1.0 - smoothstep(0.995, 1.0, lightToPosDepth_ws));
}  