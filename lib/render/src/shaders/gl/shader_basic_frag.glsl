#version 330 core
out vec4 FragColor;

uniform vec4 u_Color;
in vec3 outColor;
in vec2 outTexCoord;

uniform sampler2D u_Texture;

void main()
{
    //FragColor = u_Color;
    //FragColor = vec4(outColor.xyz + u_Color.xyz, 1.0);
    FragColor = texture(u_Texture, outTexCoord);
}