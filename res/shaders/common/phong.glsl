#include "res/shaders/common/materialDef.glsl"

struct Light {
    vec3 ambientCol;
    float ambientStrength;
    
    vec3 diffuseCol;
    float diffuseStrength;
    
    vec3 specularCol;
    float specularStrength;

    // for spotlights
    float innerCutoff; // cosine value
    float outerCutoff; // cosine value

    // for light attenuation
    // Intensity = Ad^2 + Bd + C
    float quadratic;
    float linear;
    float constant;

    vec3 position;
    vec3 direction;
};

// Returns a float between 0.0 and 1.0 
// representing resultant light when attenuated based on distance from source
float calcLightAttenutation(Light light, vec3 fragPos) {
    // get the vector from the light position to the pixel and calculate the magnitude
    // using the length function
    float lightDist = length(light.position - fragPos);

    // put that distance into a quadratic function for non-linear degradation
    float lightAttenuation = 1.0 / (lightDist * lightDist * light.quadratic + lightDist * light.linear + light.constant);
    
    return lightAttenuation;
}

vec3 calcAmbientLight(Light light) {
    return light.ambientStrength * light.ambientCol;
}

// Diffuse light is based on angle between light ray and surface normal
vec3 calcDiffuseLight(Light light, vec3 fragPos, vec3 normal) {
    // get the vector from the light position to the pixel
    vec3 lightRayDir = normalize(light.position - fragPos); 

    // get cosine of angle between the normal and lightray direction
    // the steeper the angle, the lower the value
    float diff = max(dot(normal, lightRayDir), 0.0);

    vec3 diffLight = light.diffuseStrength * diff * light.diffuseCol;
    return diffLight;
}

// Diffuse light is based on angle between light ray and surface normal.
// However, this assumes that the light ray is always perpendicular to the actual light's
// pointing direction
vec3 calcDiffuseLightDir(Light light, vec3 fragPos, vec3 normal) {
    // because this is used for directional lighting, the light rays are parrallel and fixed
    vec3 lightRayDir = normalize(-light.direction); 

    // get cosine of angle between the normal and lightray direction
    // the steeper the angle, the lower the value
    float diff = max(dot(normal, lightRayDir), 0.0);

    vec3 diffLight = light.diffuseStrength * diff * light.diffuseCol;
    return diffLight;
}

// Specular light is based on angle between light ray reflection and view direction
vec3 calcSpecularLight(Light light, Material material, vec3 fragPos, vec3 normal, vec3 camPos) {
    // get the vector from the camera position to the pixel
    vec3 viewDir = normalize(camPos - fragPos);

    // reflect that vector on the surface (which is perpendicular to the normal)
    // note that the reflect function expects the vector to be pointing away (hence negate)
    vec3 reflectDir = reflect(-normalize(light.position - fragPos), normal);  

    // magic...
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specLight = light.specularStrength * spec * light.specularCol;  
    return specLight;
}

// Specular light is based on angle between the surface normal and angle that is midway between view direction
// and light ray direction
vec3 calcBlinnSpecularLight(Light light, Material material, vec3 fragPos, vec3 normal, vec3 camPos) {
    // get the vector from the camera position to the pixel
    vec3 viewDir = normalize(camPos - fragPos);
    vec3 lightDir = normalize(light.position - fragPos);
    vec3 halfwayDir = normalize(viewDir + lightDir);

    // magic...
    float spec = pow(max(dot(normal, halfwayDir), 0.0), material.shininess);
    vec3 specLight = light.specularStrength * spec * light.specularCol;  
    return specLight;
}

// Maintains a fixed light ray direction that is perpendicular to the actual light's
// pointing direction during the diffuse light calculation
vec3 calcPhongDirLight(Light light, Material material, vec3 fragPos, vec3 normal, vec3 camPos) {
    vec3 ambientLight = calcAmbientLight(light);
    vec3 diffuseLight = calcDiffuseLightDir(light, fragPos, normal);
    vec3 specularLight = calcSpecularLight(light, material, fragPos, normal, camPos);
    return ambientLight + diffuseLight + specularLight;
}

vec3 calcPhongPointLight(Light light, Material material, vec3 fragPos, vec3 normal, vec3 camPos) {
    float lightAttenuation = calcLightAttenutation(light, fragPos);
    vec3 ambientLight = calcAmbientLight(light);
    vec3 diffuseLight = calcDiffuseLight(light, fragPos, normal);
    vec3 specularLight = calcSpecularLight(light, material, fragPos, normal, camPos);
    return lightAttenuation * (ambientLight + diffuseLight + specularLight);
}

// Applies cut-off at radius of light's pointing direction
vec3 calcPhongSpotlight(Light light, Material material, vec3 fragPos, vec3 normal, vec3 camPos) {
    // get the vector from the light position to the pixel
    vec3 lightRayDir = normalize(light.position - fragPos); 

    // use dot product to get cosine between direction that the actual light is pointing and
    // the vector of the light to the pixel.
    // if that angle is larger than our cutoff, we do not apply the cutoff. 
    float theta = dot(lightRayDir, normalize(-light.direction));


    // from 0-90 degrees, cosine values actually get smaller. So if the cosine value is larger, 
    // the angle is smaller. So we do inner - outer for this reason.
    float epsilon = light.innerCutoff - light.outerCutoff;

    float intensity = clamp((theta - light.outerCutoff) / epsilon, 0.0, 1.0);

    // we don't ever apply the cutoff to ambient as there should always be ambient light outside
    // of the cutoff
    float lightAttenuation = calcLightAttenutation(light, fragPos);
    vec3 ambientLight = calcAmbientLight(light);
    vec3 diffuseLight = intensity * calcDiffuseLight(light, fragPos, normal);
    vec3 specularLight = intensity * calcSpecularLight(light, material, fragPos, normal, camPos);

    return lightAttenuation * (ambientLight + diffuseLight + specularLight);
}
