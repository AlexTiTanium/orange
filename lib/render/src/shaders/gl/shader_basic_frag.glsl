#version 330 core
out vec4 FragColor;

uniform vec4 u_Color;
in vec3 outColor;

void main()
{
    //FragColor = u_Color;
    FragColor = vec4(outColor, 1.0);
}