#version 150


in vec3 normal;
in vec3 position;
in vec3 color;
in vec2 tex_coords;

out vec3 v_normal;
out vec3 v_position;
out vec3 v_color;
out vec2 v_tex_coords;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;


void main() {
    mat4 modelview = view * model;
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    gl_Position = perspective * modelview * vec4(position, 1.0);
    v_position = gl_Position.xyz / gl_Position.w;
    float new_color = (color.x + color.y + color.z) / 3.0 > 0.5 ? 1.0 : 0.0;
    v_color = vec3(new_color, new_color, new_color);
    v_tex_coords = tex_coords;
}