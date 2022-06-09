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