#shader vertex
#version 450 core
#include "res/shaders/common/buffers/lightsBuffer.glsl"
#include "res/shaders/common/buffers/materialsBuffer.glsl"
#include "res/shaders/common/buffers/matricesBuffer.glsl"
#include "res/shaders/common/util.glsl"

#line 0 12

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec3 a_tangent;
layout(location = 3) in vec4 a_colour;
layout(location = 4) in vec2 a_texCoord;

layout(location = 5) in uint a_materialIndex;
layout(location = 6) in mat4 a_transform;

out OBJECT {
    vec2 texCoord;
    flat Material material;
} obj_out;

out LIGHT_SPACE {
    // vec3 spotLightToPos[MAX_SPOT_LIGHTS];
    vec3 dirLightToPos;
} ls_out;

out WORLD_SPACE {
    vec3 pos;

    // vec3 pointLightToPos[MAX_POINT_LIGHTS];
    // float pointLightToPosDepth[MAX_POINT_LIGHTS];
} ws_out;

out TANGENT_SPACE {
    vec3 pos;
    vec3 normal;

    vec3 toCam;

    // vec3 toPointLight[MAX_POINT_LIGHTS];
    // vec3 toSpotLight[MAX_SPOT_LIGHTS];
    vec3 toDirLight;
} ts_out;


void main() {
    vec3 T = normalize(vec3(a_transform * vec4(a_tangent, 0.0)));
    vec3 N = normalize(vec3(a_transform * vec4(a_normal, 0.0)));
    // re-orthogonalize T with respect to N
    T = normalize(T - dot(T, N) * N);
    // then retrieve perpendicular vector B with the cross product of T and N
    vec3 B = cross(N, T);
    mat3 tangentToWorldSpace = mat3(T, B, N);
    mat3 worldToTangentSpace = transpose(tangentToWorldSpace);

    obj_out.texCoord = a_texCoord;
    obj_out.material = materials[a_materialIndex];

    ws_out.pos = vec3(a_transform * vec4(a_position, 1.0));

    ts_out.pos = worldToTangentSpace * ws_out.pos;
    ts_out.normal = worldToTangentSpace * N;
    ts_out.toCam = worldToTangentSpace * (cameraPos - ws_out.pos);

    // for (int i = 0; i < pointLightCount; i++) {
    //     ws_out.pointLightToPos[i] = ws_out.pos - pointLights[i].position;
    //     ws_out.pointLightToPosDepth[i] = VectorToDepthValue(ws_out.pointLightToPos[i]);
    //     ts_out.toPointLight[i] = worldToTangentSpace * (pointLights[i].position - ws_out.pos);
    // }

    // for (int i = 0; i < spotLightCount; i++) {
    //     vec4 spotLightToPos = spotLights[i].view * vec4(ws_out.pos, 1.0);
    //     ls_out.spotLightToPos[i] = spotLightToPos.xyz / spotLightToPos.w;

    //     ts_out.toSpotLight[i] = worldToTangentSpace * (spotLights[i].position - ws_out.pos);
    // }

    if (directionalLightCount == 1) {
        vec4 dirLightToPos = directionalLight.view * vec4(ws_out.pos, 1.0);
        ls_out.dirLightToPos = dirLightToPos.xyz / dirLightToPos.w;

        ts_out.toDirLight = worldToTangentSpace * directionalLight.direction;
    }
 

    gl_Position = projection * view * a_transform * vec4(a_position, 1.0);
}


///////////////////////////////////////////////////////////////////////////////////////
#shader fragment
#version 450 core
#extension GL_ARB_bindless_texture : require
#include "res/shaders/common/defs/lights.glsl"
#include "res/shaders/common/defs/material.glsl"
#include "res/shaders/common/buffers/lightsBuffer.glsl"
#include "res/shaders/common/buffers/shadowMapsBuffer.glsl"
#include "res/shaders/common/phong.glsl"

#line 0 13

in OBJECT {
    vec2 texCoord;
    flat Material material;
} obj_in;

in LIGHT_SPACE {
    // vec3 spotLightToPos[MAX_SPOT_LIGHTS];
    vec3 dirLightToPos;
} ls_in;

in WORLD_SPACE {
    vec3 pos;

    // vec3 pointLightToPos[MAX_POINT_LIGHTS];
    // float pointLightToPosDepth[MAX_POINT_LIGHTS];
} ws_in;

in TANGENT_SPACE {
    vec3 pos;
    vec3 normal;

    vec3 toCam;

    // vec3 toPointLight[MAX_POINT_LIGHTS];
    // vec3 toSpotLight[MAX_SPOT_LIGHTS];
    vec3 toDirLight;
} ts_in;

out vec4 FragColor;

void main() {
    vec3 normalMap = texture(sampler2D(obj_in.material.normalTexture), obj_in.texCoord).rgb;
    normalMap = normalize(normalMap * 2.0 - 1.0);   

    vec3 diffuseTexture = texture(sampler2D(obj_in.material.diffuseTexture), obj_in.texCoord).rgb;
    float gloss = texture(sampler2D(obj_in.material.specularTexture), obj_in.texCoord).r; 


    vec3 totalLight = vec3(0.0); 

    // for (int i = 0; i < pointLightCount; i++) {
    //     totalLight += calcBlinnPhongPointLight(
    //                         pointLights[i], 
    //                         obj_in.material, 
    //                         ts_in.toPointLight[i], 
    //                         ts_in.toCam,
    //                         ws_in.pointLightToPos[i],
    //                         ws_in.pointLightToPosDepth[i],
    //                         samplerCubeShadow(pointShadowMaps[i]), 
    //                         normalMap, 
    //                         gloss
    //                     );
    // }


    // for (int i = 0; i < spotLightCount; i++) {
    //     totalLight += calcBlinnPhongSpotlight(
    //                         spotLights[i], 
    //                         obj_in.material, 
    //                         ts_in.toSpotLight[i], 
    //                         ts_in.toCam, 
    //                         ls_in.spotLightToPos[i],
    //                         sampler2D(spotShadowMaps[i]), 
    //                         normalMap, 
    //                         gloss
    //                     );
    // }

    if (directionalLightCount == 1)  {
        totalLight += calcBlinnPhongDirLight(
                            directionalLight, 
                            obj_in.material, 
                            ts_in.toDirLight, 
                            ts_in.toCam, 
                            ls_in.dirLightToPos,
                            sampler2D(directionalShadowMap), 
                            normalMap, 
                            gloss
                         );
    }

    FragColor = vec4(diffuseTexture * totalLight, 1.0);
}