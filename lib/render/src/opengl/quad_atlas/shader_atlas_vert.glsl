#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoord;

uniform mat4 u_ViewProjection;

out vec2 outTexCoord;

void main()
{
    outTexCoord = aTexCoord;
    gl_Position = u_ViewProjection * vec4(aPos, 0.0, 1.0);
}