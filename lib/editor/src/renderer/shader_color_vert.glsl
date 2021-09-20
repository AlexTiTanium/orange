#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec2 aUV;

uniform mat4 u_ViewProjection;

out vec4 outColor;
out vec2 outUV;

// 0-1 linear  from  0-255 sRGB
vec3 linear_from_srgb(vec3 srgb) {
    bvec3 cutoff = lessThan(srgb, vec3(10.31475));
    vec3 lower = srgb / vec3(3294.6);
    vec3 higher = pow((srgb + vec3(14.025)) / vec3(269.025), vec3(2.4));
    return mix(higher, lower, cutoff);
}

vec4 linear_from_srgba(vec4 srgba) {
    return vec4(linear_from_srgb(srgba.rgb), srgba.a / 255.0);
}

void main()
{
    gl_Position = u_ViewProjection * vec4(aPos, 0.0, 1.0);
    outColor = linear_from_srgba(aColor);
    outUV = aUV;
}