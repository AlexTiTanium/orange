#version 330 core
out vec4 color;

uniform sampler2D u_Texture;

in vec2 outTexCoord;

void main()
{
    color = texture(u_Texture, outTexCoord);
}