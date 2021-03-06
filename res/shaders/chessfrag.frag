#version 150
in vec2 v_uv;
in vec4 v_color;

uniform sampler2D u_texture;
out vec4 o_color;

uniform vec2 viewport;
void main() {
    o_color = v_color * texture(u_texture, v_uv) * vec4(0.0, 0.0, 0.0, 1.0);
}