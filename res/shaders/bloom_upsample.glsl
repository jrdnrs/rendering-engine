#shader compute
#version 450 core
#include "res/shaders/common/buffers/generalBuffer.glsl"

#define READ_IMAGE_SIZE         vector1.xy
#define READ_MIP_LEVEL          index1

#define BLOOM_INTENSITY         1.33

#define GROUP_WIDTH             16
#define GROUP_COUNT             (GROUP_WIDTH * GROUP_WIDTH)
#define SAMPLE_RADIUS           2
#define GROUP_SAMPLES_WIDTH     (GROUP_WIDTH + (SAMPLE_RADIUS * 2))
#define GROUP_SAMPLES_COUNT     (GROUP_SAMPLES_WIDTH * GROUP_SAMPLES_WIDTH)

layout (local_size_x = GROUP_WIDTH, local_size_y = GROUP_WIDTH, local_size_z = 1) in;
layout(binding = 0) uniform sampler2D read_image;
layout(rgba16f, binding = 1) uniform image2D write_image;

shared vec3 samples[GROUP_SAMPLES_COUNT];

void main() {
    // each thread is responsible for writing to a unique pixel, so use global ID as the coordinates for that
    ivec2 texelCoord = ivec2(gl_GlobalInvocationID);

    // as we will need to sample pixels from outside of this thread group (depending on the sample radius), we need
    // to populate a larger grid of samples which is then stored in 1D array of shared memory
    // the 2 grids (pixel grid/thread group and the sample grid) should be centred around the same point, so we need 
    // to offset the texelCoord for the sample to achieve this 
    ivec2 sampleTexelCoord = ivec2(gl_WorkGroupID) * GROUP_WIDTH - SAMPLE_RADIUS;

    // collection of samples is distributed evenly amongst threads
    // we use the flat ID (gl_LocalInvocationIndex) of the current thread within the group and reconstruct the offset
    // to be used for the sample grid, as its width may be different to the width of the thread group
    for (int i = int(gl_LocalInvocationIndex); i < GROUP_SAMPLES_COUNT; i += GROUP_COUNT) {

        ivec2 currentSampleTexelCoord = sampleTexelCoord + ivec2(i % GROUP_SAMPLES_WIDTH, i / GROUP_SAMPLES_WIDTH);
        vec2 normalisedTexelCoord = (vec2(currentSampleTexelCoord) + 0.5) / READ_IMAGE_SIZE;

        samples[i] = textureLod(read_image, normalisedTexelCoord, READ_MIP_LEVEL).rgb;
    }

    groupMemoryBarrier();

    uint middleIndex = (gl_LocalInvocationID.x + SAMPLE_RADIUS) + (gl_LocalInvocationID.y + SAMPLE_RADIUS) * GROUP_SAMPLES_WIDTH;
    
    #define x    1 
    #define y    GROUP_SAMPLES_WIDTH
    #define rx   (x * SAMPLE_RADIUS)
    #define ry   (y * SAMPLE_RADIUS)

    // Take 9 samples around current texel:
    // a - b - c
    // d - e - f
    // g - h - i
    vec3 a = samples[middleIndex - ry + rx];
    vec3 b = samples[middleIndex      + rx];
    vec3 c = samples[middleIndex + ry + rx];

    vec3 d = samples[middleIndex - ry];
    vec3 e = samples[middleIndex];
    vec3 f = samples[middleIndex + ry];

    vec3 g = samples[middleIndex - ry - rx];
    vec3 h = samples[middleIndex      - rx];
    vec3 i = samples[middleIndex + ry - rx];

    // Apply weighted distribution, by using a 3x3 tent filter:
    //  1   | 1 2 1 |
    // -- * | 2 4 2 |
    // 16   | 1 2 1 |
    vec3 upsample = e*4.0;
    upsample += (b+d+f+h)*2.0;
    upsample += (a+c+g+i);
    upsample /= 16.0;
    upsample *= BLOOM_INTENSITY;
    vec3 before = imageLoad(write_image, texelCoord).rgb;
    vec3 result = before + upsample;

    imageStore(write_image, texelCoord, vec4(result, 1.0));
}

