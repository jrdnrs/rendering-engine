struct PointLight {
    vec3 ambientCol;
    vec3 diffuseCol;
    vec3 specularCol;

    // for light attenuation
    // quadratic, linear, constant
    // Intensity = Ad^2 + Bd + C
    vec3 attenuation;

    vec3 position;

    mat4 views[6];
};

struct SpotLight {
    vec3 ambientCol;
    vec3 diffuseCol;
    vec3 specularCol;

    // for light attenuation
    // quadratic, linear, constant
    // Intensity = Ad^2 + Bd + C
    vec3 attenuation;

    float innerCutoff; // cosine value
    float outerCutoff; // cosine value

    vec3 position;
    vec3 direction;

    mat4 view;
};

struct DirectionalLight {
    vec3 ambientCol;
    vec3 diffuseCol;
    vec3 specularCol;

    vec3 position;
    vec3 direction;

    mat4 view;
};