#shader vertex
#version 460 core
#include "res/shaders/common/materialDef.glsl"

layout(location = 0) in vec3 a_position;
layout(location = 1) in vec3 a_normal;
layout(location = 2) in vec4 a_colour;
layout(location = 3) in vec2 a_texCoord;
layout(location = 4) in int a_index;

out VS_OUT {
    vec3 fragPos; 
    vec3 normal;
    vec2 texCoord;
    Material material;
} vs_out;

layout (std430, binding = 0) buffer Vertex {
    Material materials[100];
    mat4 transforms[1000];
    mat4 projection;
    mat4 view;
    uint materialIndex[1000];
};

void main() {
    mat4 transform = transforms[a_index];

    vs_out.fragPos = vec3(transform * vec4(a_position, 1.0));
    vs_out.normal = mat3(transpose(inverse(transform))) * a_normal;
    vs_out.texCoord = a_texCoord;
    vs_out.material = materials[materialIndex[a_index]];

    gl_Position = projection * view * transform * vec4(a_position, 1.0);
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
} vs_in;

out vec4 FragColour;

layout (std430, binding = 1) buffer Fragment {
    Light allLights[32];
    int lightCount;
    vec3 cameraDir;
    vec3 cameraPos;
};

void main() {
//     Material material = Material(
//          vec3(0.95, 0.85, 0.6),
//          vec3(0.86, 0.77, 0.54),
//          vec3(0.36, 0.33, 0.3),
//          8.0
//     );

//    Light lamp = Light(
//         vec3(3.0, -3.0, -3.0),
//         vec3(0.0, 1.0, 0.0),
//         0.3, 1.33, 0.5,
//         1.0, 1.0, // cutoff not needed for point light
//         0.0075, 0.045, 1.0
//    );

//     Light sun = Light(
//         vec3(3.0, -3.0, -3.0),
//         vec3(0.0, 1.0, 0.0),
//         0.3, 1.33, 0.6,
//         1.0, 1.0, // cutoff not needed for directional light
//         1.0, 1.0, 1.0 // attenuation not needed for directional light
//    );

//     Light flashlight = Light(
//         cameraPos,
//         cameraDir,
//         0.3, 1.33, 0.8,
//         0.9863, 0.8892,
//         0.0075, 0.045, 1.0
//    );

//     vec3 phongLight = calcPhongPointLight(lamp, material, FragPos, cameraPos);
//     vec3 dirLight = calcPhongDirLight(sun, material, FragPos, cameraPos);
//     vec3 spotlight = calcPhongSpotlight(flashlight, material, FragPos, cameraPos);


    vec3 myColour = vec3(texture(sampler2D(vs_in.material.diffuseTexture), vs_in.texCoord));
    vec3 totalLight = vec3(0.0);

    for (int i = 0; i < lightCount; i++) {
        float lightAttenuation = calcLightAttenutation(allLights[i], vs_in.fragPos);
        vec3 ambientLight = myColour * calcAmbientLight(allLights[i], vs_in.material);
        vec3 diffuseLight = myColour * calcDiffuseLight(allLights[i], vs_in.material, vs_in.fragPos, vs_in.normal);
        vec3 specularLight = myColour * calcSpecularLight(allLights[i], vs_in.material, vs_in.fragPos, vs_in.normal, cameraPos);

        totalLight += lightAttenuation * (ambientLight + diffuseLight + specularLight);
    }
    
    FragColour = vec4(totalLight, 1.0);
}