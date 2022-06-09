#shader vertex
#version 460 core
#include "res/shaders/common/defs/material.glsl"
#include "res/shaders/common/defs/light.glsl"

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec4 a_colour;
layout(location = 3) in vec2 a_texCoord;

layout(location = 4) in int a_materialIndex;
layout(location = 5) in mat4 a_transform;

out VS_OUT {
    vec3 fragPos; 
    vec3 normal;
    vec2 texCoord;
    Material material;
    vec3 fragPosLightSpace;
} vs_out;

layout (std430, binding = 0) buffer Lights {
    Light allLights[32];
    mat4 lightViews[32];
    mat4 lightProjection;
    uvec2 shadowMaps[32];
    int lightCount;
    vec3 cameraDir;
    vec3 cameraPos;
};

layout (std430, binding = 2) buffer Materials {
    Material materials[100];
};

layout (std430, binding = 3) buffer Matrices {
    mat4 projection;
    mat4 view;
};

void main() {
    vs_out.fragPos = vec3(a_transform * vec4(a_position, 1.0));
    vs_out.normal = mat3(transpose(inverse(a_transform))) * a_normal;
    vs_out.texCoord = a_texCoord;
    vs_out.material = materials[a_materialIndex];

    vec4 fragPosLightSpace = lightProjection * lightViews[0] * vec4(vs_out.fragPos, 1.0);
    vs_out.fragPosLightSpace = fragPosLightSpace.xyz / fragPosLightSpace.w;
    gl_Position = projection * view * a_transform * vec4(a_position, 1.0);
}


///////////////////////////////////////////////////////////////////////////////////////
#shader fragment
#version 460 core
#extension GL_ARB_bindless_texture : require
#include "res/shaders/common/phong.glsl"

in VS_OUT {
    vec3 fragPos; 
    vec3 normal;
    vec2 texCoord;
    Material material;
    vec3 fragPosLightSpace;
} vs_in;

out vec4 FragColour;

layout (std430, binding = 0) buffer Lights {
    Light allLights[32];
    mat4 lightViews[32];
    mat4 lightProjection;
    uvec2 shadowMaps[32];
    int lightCount;
    vec3 cameraDir;
    vec3 cameraPos;
};


float ShadowCalculation(vec3 fragPosLightSpace, vec3 lightDir, vec3 normal)
{

    sampler2D shadowMap = sampler2D(shadowMaps[0]);

    // transform to [0,1] range
    vec3 projCoords = clamp(fragPosLightSpace * 0.5 + 0.5, 0.0, 1.0);
    float currentDepth = projCoords.z;
    
    float bias = 0.001;  
    // float bias = max(0.05 * (1.0 - dot(normal, lightDir)), 0.005);  

    float shadow = 0.0;
    vec2 texelSize = 1.0 / textureSize(shadowMap, 0);

    // radius to sample around centre pixel
    int radius = 1;
    for (int x = -radius; x <= radius; x++)  {
        for (int y = -radius; y <= radius; y++)  {
            float closestDepth = texture(shadowMap, projCoords.xy + vec2(x, y) * texelSize).r; 
            shadow += currentDepth > closestDepth ? 1.0 : 0.0;  
        }
    }
    shadow /= pow((radius * 2 + 1), 2);

    return shadow;
}  

void main() {
    vec3 myColour = vec3(texture(sampler2D(vs_in.material.diffuseTexture), vs_in.texCoord));
    vec3 totalLight = vec3(0.0);

    for (int i = 0; i < lightCount; i++) {
        float shadow = 1.0 - ShadowCalculation(vs_in.fragPosLightSpace, allLights[i].direction, vs_in.normal);
        float lightAttenuation = calcLightAttenutation(allLights[i], vs_in.fragPos);
        vec3 ambientLight = myColour * calcAmbientLight(allLights[i]);
        vec3 diffuseLight = myColour * calcDiffuseLight(allLights[i], vs_in.fragPos, vs_in.normal);
        vec3 specularLight = vec3(0.0); 

        vec3 lightDir = normalize(allLights[i].position - vs_in.fragPos);
        float cosine = dot(vs_in.normal, lightDir);
        float angle = 180.0 - (cosine + 1.0) / 2.0 * 180.0;

        if (angle > 90.0) {
            ambientLight = ambientLight * (1.0 - (angle / 180.0 / 1.15) );
        }

        if (angle <= 90.0) {
            specularLight = myColour * calcBlinnSpecularLight(allLights[i], vs_in.material, vs_in.fragPos, vs_in.normal, cameraPos);
        }

        totalLight += lightAttenuation * (ambientLight + (diffuseLight * shadow) + (specularLight * shadow));
    }
    
    FragColour = vec4(totalLight, 1.0);
}