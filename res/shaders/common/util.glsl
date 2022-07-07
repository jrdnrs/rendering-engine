// https://stackoverflow.com/questions/10786951/omnidirectional-shadow-mapping-with-depth-cubemap

#line 0 11

float VectorToDepthValue(vec3 Vec) {
    const float F = 20.0;
    const float N = 1.0;

    vec3 AbsVec = abs(Vec);
    float LocalZcomp = max(AbsVec.x, max(AbsVec.y, AbsVec.z));

    float NormZComp = (F+N) / (F-N) - (2*F*N)/(F-N)/LocalZcomp;
    return (NormZComp + 1.0) * 0.5;
}

float InterleavedGradientNoise(vec2 screenPosition) {
    const vec3 MAGIC = vec3(0.06711056, 0.00583715, 52.9829189);

    return fract(MAGIC.z * fract(dot(screenPosition, MAGIC.xy)));
}

vec2 VogelDisk(int sampleIndex, int sampleCount, float phi) {
    const float GOLDEN_ANGLE = 2.4;

    float r = sqrt(sampleIndex + 0.5) / sqrt(sampleCount);
    float theta = sampleIndex * GOLDEN_ANGLE + phi;
    float sine = sin(theta);
    float cosine = cos(theta);

    return r * vec2(cosine, sine);
}