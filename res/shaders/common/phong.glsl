#include "res/shaders/common/defs/lights.glsl"
#include "res/shaders/common/defs/material.glsl"
#include "res/shaders/common/shadow.glsl"

#line 0 14

// Returns a float between 0.0 and 1.0 
// representing resultant light when attenuated based on distance from source
float calcLightAttenuation(vec3 toLight_ts, vec3 attenuation) {
    // get the vector from the light position to the pixel and calculate the magnitude
    // using the length function
    float lightDist = length(toLight_ts);

    // put that distance into a quadratic function for non-linear degradation
    float lightAttenuation = 1.0 / (lightDist * lightDist * attenuation.x + 
                                    lightDist * attenuation.y + 
                                    attenuation.z);
    
    return lightAttenuation;
}


// Cosine of angle between light ray (from lightPos to fragPos) and surface normal
// value between 0.0 and 1.0 which represents angles 90.0 to 0.0 in degrees
float calcLightCos(vec3 toLight_ts, vec3 normal) {
    // get the vector from the light position to the pixel
    vec3 lightRayDir = normalize(toLight_ts); 

    // get cosine of angle between the normal and lightray direction
    // the steeper the angle, the lower the value
    float diff = max(dot(normal, lightRayDir), 0.0);
    return diff;
}

float calcAmbientLight(float strength) {
    return strength;
}

// Diffuse light is based on angle between light ray and surface normal
float calcDiffuseLight(float lightCos) {
    return lightCos;
}

// Specular light is based on angle between light ray reflection and view direction
vec3 calcSpecularLight(vec3 toCam_ts, vec3 toLight_ts, vec3 normal, vec3 colour, Material material) {
    // get the vector from the camera position to the pixel
    vec3 viewDir = normalize(toCam_ts);

    // reflect that vector on the surface (which is perpendicular to the normal)
    // note that the reflect function expects the vector to be pointing away (hence negate)
    vec3 reflectDir = reflect(-normalize(toLight_ts), normal);  

    float spec = pow(max(dot(viewDir, reflectDir), 0.0), material.shininess);
    vec3 specLight = spec * colour;  
    return specLight;
}

// Specular light is based on angle between the surface normal and angle that is midway between view direction
// and light ray direction
float calcBlinnSpecularLight(vec3 toCam_ts, vec3 toLight_ts, vec3 normal, float shininess) {
    // get the vector from the camera position to the pixel
    vec3 viewDir = normalize(toCam_ts);
    vec3 lightRayDir = normalize(toLight_ts);
    vec3 halfwayDir = normalize(viewDir + lightRayDir);

    float exponent = exp2(shininess * 10.0) + 1.0;

    float spec = pow(max(dot(normal, halfwayDir), 0.0), exponent) * shininess;
    return spec;
}


vec3 calcBlinnPhongPointLight(PointLight light, Material material, vec3 toLight_ts, vec3 toCam_ts, vec3 lightToPos_ws,
                            float lightToPosDepth_ws, samplerCubeShadow shadowMap, vec3 normal_ts, float gloss) {
    float lightAttenuation = calcLightAttenuation(toLight_ts, light.attenuation);

    float shadow = 1.0 - calcOmniShadow(shadowMap, lightToPos_ws, lightToPosDepth_ws);
    float lightCos = calcLightCos(toLight_ts, normal_ts);

    float ambientLight = calcAmbientLight(0.05);
    float diffuseLight = shadow * calcDiffuseLight(lightCos);
    float specularLight = lightCos * shadow * calcBlinnSpecularLight(toCam_ts, toLight_ts, normal_ts, gloss);

    return lightAttenuation * light.diffuseCol * (ambientLight + diffuseLight + specularLight);
}

// Applies cut-off at radius of light's pointing direction
vec3 calcBlinnPhongSpotlight(SpotLight light, Material material, vec3 toLight_ts, vec3 toCam_ts, vec3 spotLightToPos_ls, 
                                sampler2D shadowMap, vec3 normal_ts, float gloss) {
    // get the vector from the light position to the pixel
    vec3 lightRayDir = normalize(toLight_ts); 

    // use dot product to get cosine between direction that the actual light is pointing and
    // the vector of the light to the pixel.
    // if that angle is larger than our cutoff, we do not apply the cutoff. 
    float theta = dot(lightRayDir, normalize(light.direction));
    // FIXME: This light.direction above will not be in tangent space! do not forget

    // from 0-90 degrees, cosine values actually get smaller. So if the cosine value is larger, 
    // the angle is smaller. So we do inner - outer for this reason.
    float epsilon = light.innerCutoff - light.outerCutoff;

    float intensity = clamp((theta - light.outerCutoff) / epsilon, 0.0, 1.0);

    // we don't ever apply the cutoff to ambient as there should always be ambient light outside
    // of the cutoff
    float lightAttenuation = calcLightAttenuation(toLight_ts, light.attenuation);
    float lightCos = calcLightCos(toLight_ts, normal_ts);
    float shadow = 1.0 - calcDirShadow(shadowMap, spotLightToPos_ls, lightCos);

    float ambientLight = calcAmbientLight(0.05);
    float diffuseLight = shadow * intensity * calcDiffuseLight(lightCos);
    float specularLight = lightCos * shadow * intensity * calcBlinnSpecularLight(toCam_ts, toLight_ts, normal_ts, gloss);

    return lightAttenuation * light.diffuseCol * (ambientLight + diffuseLight + specularLight);
}


// Maintains a fixed light ray direction that is perpendicular to the actual light's
// pointing direction during the diffuse light calculation
vec3 calcBlinnPhongDirLight(DirectionalLight light, Material material, vec3 toLight_ts, vec3 toCam_ts, vec3 dirLightToPos_ls,
                             sampler2D shadowMap, vec3 normal_ts, float gloss ) {
    float lightCos = calcLightCos(toLight_ts, normal_ts);
    float shadow = 1.0 - calcDirShadow(shadowMap, dirLightToPos_ls, lightCos);

    float ambientLight = calcAmbientLight(0.05);
    float diffuseLight = shadow * calcDiffuseLight(lightCos);
    float specularLight = lightCos * shadow * calcBlinnSpecularLight(toCam_ts, toLight_ts, normal_ts, gloss);

    return light.diffuseCol * (ambientLight + diffuseLight + specularLight);
}