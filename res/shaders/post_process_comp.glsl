#shader compute
#version 450 core

#define GAMMA           2.2
#define EXPOSURE        1.0

#define GROUP_WIDTH     16

layout (local_size_x = GROUP_WIDTH, local_size_y = GROUP_WIDTH, local_size_z = 1) in;
layout(rgba16f, binding = 0) uniform image2D write_image;

void main() {
    ivec2 texelCoord = ivec2(gl_GlobalInvocationID);

    vec3 hdrColor = imageLoad(write_image, texelCoord).rgb;

    // exposure tone mapping
    vec3 mapped = vec3(1.0) - exp(-hdrColor * EXPOSURE);
    // gamma correction 
    mapped = pow(mapped, vec3(1.0 / GAMMA));

    imageStore(write_image, texelCoord, vec4(mapped, 1.0));
}