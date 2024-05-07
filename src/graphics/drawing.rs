use std::f32::consts::PI;

use glium::{glutin::surface::WindowSurface, uniform, Display, Surface};

use crate::{matrix::{Matrix, Vector}, Data, Object};

pub struct Drawing {
    pub t: f32,
    pub program: Option<glium::Program>,
}

impl Drawing {
    pub fn new() -> Self {
        Self {t: 0.0, program: None}
    }

    pub fn compute_program(&mut self, display: &Display<WindowSurface>, vertex_path: &String, fragment_path: &String) {
        let vertex_shader_src = std::fs::read_to_string(vertex_path).unwrap();
        let fragment_shader_src = std::fs::read_to_string(fragment_path).unwrap();
        let program = glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap();
        self.program = Some(program);
    }

    pub fn view_matrix(&self, position: &Vector<f32>, direction: &Vector<f32>, up: &Vector<f32>) -> Matrix<f32> {
        let f = direction.rnormalize();
    
        let mut s = Vector::cross_product(&up, &f);
    
        s.normalize();
    
        let u = Vector::cross_product(&f, &s);
    
        let p = Vector::from(&[-position[0] * s[0] - position[1] * s[1] - position[2] * s[2],
                                                    -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
                                                    -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]]);
    
        Matrix::from(&[
            &[s[0], u[0], f[0], 0.0],
            &[s[1], u[1], f[1], 0.0],
            &[s[2], u[2], f[2], 0.0],
            &[p[0], p[1], p[2], 1.0],
        ])
    }

    pub fn perspective_matrix(&self, fov: f32, ratio: f32, znear: f32, zfar: f32) -> Matrix<f32> {

        let f = 1.0 / (fov / 2.0).tan();

        Matrix::from(&[
            &[     f * ratio       ,    0.0,              0.0              ,   0.0],
            &[         0.0         ,     f ,              0.0              ,   0.0],
            &[         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            &[         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ])
    }

    pub fn draw(&mut self, display: &Display<WindowSurface>, obj: &Object, data: &Data, delta_t: &f32) {
        self.t += delta_t;
        
        let mut frame = display.draw();

        frame.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
        let (width, height) = frame.get_dimensions();
        

        let view = self.view_matrix(&data.camera.position, &data.camera.direction, &data.camera.up);

        let size_model = Matrix::from(&[
            &[1.0, 0.0, 0.0, 0.0],
            &[0.0, 1.0, 0.0, 0.0],
            &[0.0, 0.0, 1.0, 0.0],
            &[data.object_position.0, data.object_position.1, data.object_position.2, 1.0f32], // <- Translation of teapot here
        ]);

        let rotation_model = Matrix::from(&[
            &[self.t.cos(), 0.0, -self.t.sin(), 0.0],
            &[0.0, 1.0, 0.0, 0.0],
            &[self.t.sin(), 0.0, self.t.cos(), 0.0],
            &[0.0, 0.0, 0.0, 1.0f32],
        ]);

        let model = rotation_model * size_model;

        //let model = Matrix::from(&[
        //    &[self.t.cos() * 0.01, 0.0, -self.t.sin() * 0.01, 0.0],
        //    &[0.0, 0.01, 0.0, 0.0],
        //    &[self.t.sin() * 0.01, 0.0, self.t.cos() * 0.01, 0.0],
        //    &[20.0, 20.0, 0.0, 1.0f32],
        //]);
    
        let perspective = self.perspective_matrix(PI / 3.0, height as f32 / width as f32, 0.1, 1024.0);
    
        let light = [1.4, 0.4, -0.7f32];
    
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            blend: glium::draw_parameters::Blend::alpha_blending(),
            ..Default::default()
        };

        let positions = glium::VertexBuffer::new(display, &obj.vertices).unwrap();
        let normals = glium::VertexBuffer::new(display, &obj.normals).unwrap();
        let colors = glium::VertexBuffer::new(display, &obj.color).unwrap();
        let indices = glium::IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &obj.indices,
        ).unwrap();
    
        frame.draw(
                (&positions, &normals, &colors),
                &indices,
                &self.program.as_ref().unwrap(),
                &uniform! {model: model.to_list_4(), u_light: light, perspective: perspective.to_list_4(), view: view.to_list_4()},
                &params,
            )
            .unwrap();
    
        frame.finish().unwrap();        
    }
}