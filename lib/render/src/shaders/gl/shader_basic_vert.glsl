#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTexCoord;

uniform mat4 u_MVP;

out vec3 outColor;
out vec2 outTexCoord;

void main()
{
    outColor = aColor;
    outTexCoord = aTexCoord;
    gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0) * u_MVP;
}