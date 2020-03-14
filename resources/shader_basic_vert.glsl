#version 330 core
layout (location = 0) in vec2 aPos;

uniform vec4 u_Color;

void main()
{
    gl_Position = u_Color;
}