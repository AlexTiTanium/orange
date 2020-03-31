#version 330 core
out vec4 color;

uniform vec4 u_Color;
uniform sampler2D u_Texture;

in vec3 outColor;
in vec2 outTexCoord;

void main()
{
    //color = u_Color;
    //color = vec4(u_Color.xyz + outColor.xyz, 1.0);
    color = texture(u_Texture, outTexCoord);
}