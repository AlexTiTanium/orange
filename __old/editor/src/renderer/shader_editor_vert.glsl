#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec4 aColor;
layout (location = 2) in vec2 aUV;

// uniform mat4 u_ViewProjection;

out vec4 outColor;
out vec2 outUV;

void main()
{
    gl_Position = vec4(
                      2.0 * aPos.x / 1024 - 1.0,
                      1.0 - 2.0 * aPos.y / 768,
                      0.0,
                      1.0);
    outColor = aColor / 255.0;
    outUV = aUV;
}