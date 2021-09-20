#version 330 core

uniform sampler2D u_Texture;

out vec4 color;

in vec4 outColor;
in vec2 outUV;

void main()
{
    color = texture(u_Texture, outUV) * outColor;
}