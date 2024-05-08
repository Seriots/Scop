#version 450

in vec3 v_normal;
in vec3 v_position;
in vec3 v_color;

out vec4 color;

uniform vec3 u_light;

layout(std140) uniform MaterialIndex {
    vec3 index[size];
};

layout(std140) uniform Material {
    vec3 ambient_color[size];
    vec3 diffuse_color[size];
    vec3 specular_color[size];
    // vec3 emissive_color[size];
    // float specular_coef[size];
    // float alpha[size];
    // float refraction[size];
};

const int nb_colors = 5;
const vec3 colors[nb_colors] = vec3[](vec3(1.0, 1.0, 1.0), vec3(0.6, 0.6, 0.6), vec3(0.0, 0.0, 0.0), vec3(0.8, 0.8, 0.8), vec3(0.4, 0.4, 0.4));

// const vec3 ambient_color = vec3(0.0, 0.0, 0.0);
// const vec3 diffuse_color = vec3(0.64, 0.64, 0.64);
// const vec3 specular_color = vec3(0.5, 0.5, 0.5);

void main() {
    vec3 ambient = ambient_color[index[gl_PrimitiveID]];
    float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);
    vec3 camera_dir = normalize(-v_position);
    vec3 half_direction = normalize(normalize(u_light) + camera_dir);
    float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

    // color = vec4(v_color , 1.0);
    color = vec4(colors[gl_PrimitiveID % nb_colors] + diffuse * diffuse_color + specular * specular_color, 1.0);
}