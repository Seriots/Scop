use std::f32::consts::PI;

use glium::{glutin::surface::WindowSurface, uniform, Display, Surface};

use crate::{Data, Object};

pub struct Drawing {
    pub t: f32,
}

impl Drawing {
    pub fn new() -> Self {
        Self {t: 0.0}
    }

    pub fn view_matrix(&self, position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
        let f = {
            let f = direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };
    
        let s = [up[1] * f[2] - up[2] * f[1],
                 up[2] * f[0] - up[0] * f[2],
                 up[0] * f[1] - up[1] * f[0]];
    
        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        };
    
        let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
                 f[2] * s_norm[0] - f[0] * s_norm[2],
                 f[0] * s_norm[1] - f[1] * s_norm[0]];
    
        let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
                 -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
                 -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];
    
        [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }

    pub fn draw(&self, display: &Display<WindowSurface>, obj: &Object, data: &Data) {
        let mut frame = display.draw();

        frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let vertex_shader_src = r#"
            #version 150

            in vec3 normal;
            in vec3 position;

            out vec3 v_normal;
            out vec3 v_position;

            uniform mat4 perspective;
            uniform mat4 view;
            uniform mat4 model;

            void main() {
                mat4 modelview = view * model;
                v_normal = transpose(inverse(mat3(modelview))) * normal;
                gl_Position = perspective * modelview * vec4(position, 1.0);
                v_position = gl_Position.xyz / gl_Position.w;
            }
        "#;

        let fragment_shader_src = r#"
            #version 140

            in vec3 v_normal;
            in vec3 v_position;
            
            out vec4 color;

            uniform vec3 u_light;

            const vec3 ambient_color = vec3(0.2, 0.0, 0.0);
            const vec3 diffuse_color = vec3(0.6, 0.0, 0.0);
            const vec3 specular_color = vec3(1.0, 1.0, 1.0);

            void main() {
                float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);
                vec3 camera_dir = normalize(-v_position);
                vec3 half_direction = normalize(normalize(u_light) + camera_dir);
                float specular = pow(max(dot(half_direction, normalize(v_normal)), 0.0), 16.0);

                color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
            }
        "#;

        let program = glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

        let view = self.view_matrix(&[2.0, 1.0, 1.0], &[-2.0, -1.0, 1.0], &[0.0, 1.0, 0.0]);
        let model = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 2.0, 1.0f32],
        ];
    
        let perspective = {
            let (width, height) = frame.get_dimensions();
    
            let aspect_ratio = height as f32 / width as f32;
    
            let fov: f32 = PI / 3.0;
    
            let zfar = 1024.0;
            let znear = 0.1;
    
            let f = 1.0 / (fov / 2.0).tan();
    
            [
                [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
                [         0.0         ,     f ,              0.0              ,   0.0],
                [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
                [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
            ]
        };
    
        let light = [1.4, 0.4, -0.7f32];
    
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            // backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        let positions = glium::VertexBuffer::new(display, &obj.vertices).unwrap();
        let normals = glium::VertexBuffer::new(display, &obj.normals).unwrap();
        let indices = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &obj.indices,
        ).unwrap();
    
        frame.draw(
                (&positions, &normals),
                &indices,
                &program,
                &uniform! {model: model, u_light: light, perspective: perspective, view: view},
                &params,
            )
            .unwrap();
    
        frame.finish().unwrap();        
    }
}