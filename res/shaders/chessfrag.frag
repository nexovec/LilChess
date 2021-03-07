#version 150
in vec2 v_uv;
in vec4 v_color;

uniform sampler2D u_texture;
out vec4 o_color;

uniform vec2 viewport;
void main() {
    vec2 squareSize = viewport/8.0;
    if(int(floor(gl_FragCoord.x / squareSize.x) + floor(gl_FragCoord.y / squareSize.y)+1) % 2 == 0)
    // if((squareCoords.x+squareCoords.y)%2==0)
        o_color = v_color* vec4(0.0, 0.0, 0.0, 1.0);
    else
        o_color = v_color*vec4(0.6,0.6,0.6,1.0);
        // o_color = v_color * texture(u_texture, v_uv);
};