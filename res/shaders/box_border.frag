#version 150

in vec4 v_color;
in vec4 v_uv;

out vec4 o_color;

uniform vec2 viewport;
uniform int border_width;
uniform vec4 border_color;

void main(){
    bool is_border = gl_FragCoord.x < border_width || gl_FragCoord.y < border_width || gl_FragCoord.y+border_width>viewport.y || gl_FragCoord.x+border_width>viewport.x;
    o_color = int(is_border)*border_color+int(!is_border)*vec4(0.,0.,0.,0.0);
}