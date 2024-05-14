#version 450

in vec3 v_normal;
in vec3 v_position;
in vec3 v_color;
in vec3 v_tex_coords;
in vec3 v_base_normal;

out vec4 color;

uniform vec3 u_light;

uniform vec3 u_ambient_color;
uniform vec3 u_diffuse_color;
uniform vec3 u_specular_color;

uniform vec3 u_emissive_color;
uniform float u_specular_coef;
uniform float u_alpha;
uniform float u_refraction;
uniform int u_mode;
uniform float u_transition_percent;

uniform sampler2D u_texture;


const int nb_colors = 5;
const vec3 colors[nb_colors] = vec3[](vec3(1.0, 1.0, 1.0), vec3(0.6, 0.6, 0.6), vec3(0.0, 0.0, 0.0), vec3(0.8, 0.8, 0.8), vec3(0.4, 0.4, 0.4));


void main() {
    vec3 base_color = u_ambient_color;
    float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);
    vec3 camera_dir = normalize(-v_position);
    vec3 half_direction = normalize(normalize(u_light) + camera_dir);
    float specular = u_specular_coef * pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);
    
    float face1 = abs(length(cross(v_base_normal, vec3(0.0, 0.0, 1.0))));
    float face2 = abs(length(cross(v_base_normal, vec3(0.0, 1.0, 0.0))));
    float face3 = abs(length(cross(v_base_normal, vec3(1.0, 0.0, 0.0))));


    vec2 calc_text_coords = vec2((v_tex_coords.z + v_tex_coords.x), v_tex_coords.y); // <- Y FACE

    if (u_mode == 1) {
        vec3 base_color = u_ambient_color * (1.0 - u_transition_percent) + colors[gl_PrimitiveID % nb_colors] * u_transition_percent; 
        color = vec4(base_color, 1.0);
    } else if (u_mode == 2) {
        vec4 base_color = vec4(colors[gl_PrimitiveID % nb_colors] * (1.0 - u_transition_percent), 1.0) + texture(u_texture, calc_text_coords) * u_transition_percent; 
        color = base_color;
    } else {
        vec4 base_color = texture(u_texture, calc_text_coords) * (1.0 - u_transition_percent) + vec4(u_ambient_color * u_transition_percent, 1.0);
        vec3 base_color_adjusted = vec3(base_color.x, base_color.y, base_color.z);
        color = vec4(base_color_adjusted + diffuse * u_diffuse_color + specular * u_specular_color, u_alpha);
    }
}