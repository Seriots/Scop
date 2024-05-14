
use std::fs;

use glium::implement_vertex;

use crate::{get_index_by_name, matrix::Vector, Material};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: (f32, f32, f32),
    pub color: (f32, f32, f32),
    pub tex_coords: (f32, f32, f32),
}

implement_vertex!(Vertex, position, color, tex_coords);

#[derive(Copy, Clone)]
pub struct Normal {
    pub normal: (f32, f32, f32)
}

implement_vertex!(Normal, normal);

#[derive(Copy, Clone)]
pub struct Face {
    pub face: u16
}

implement_vertex!(Face, face);

#[derive(Default, Clone)]
pub struct Object {
    pub name: String,
    pub vertices: Vec<Vertex>,
    pub normals: Vec<Normal>,
    pub triangle_normals: Vec<Face>,
    pub indices: Vec<u16>,
    pub material_path: String,
    pub materials: Vec<Material>,
    pub link_material: Vec<usize>,
}

impl Object {
    fn parse_vertex(&mut self, line: &str) {
        let mut iter = line.split_whitespace();
        iter.next();
        let x = iter.next().unwrap().parse::<f32>().unwrap();
        let y = iter.next().unwrap().parse::<f32>().unwrap();
        let z = iter.next().unwrap().parse::<f32>().unwrap();
        let random_value = rand::random::<f32>();
        self.vertices.push(Vertex { position: (x, y, z), color: (random_value, random_value, random_value), tex_coords: (x, y, z)})
    }

    fn parse_normal(&mut self, line: &str) {
        let mut iter = line.split_whitespace();
        iter.next();
        let x = iter.next().unwrap().parse::<f32>().unwrap();
        let y = iter.next().unwrap().parse::<f32>().unwrap();
        let z = iter.next().unwrap().parse::<f32>().unwrap();
        self.normals.push(Normal { normal: (x, y, z) })
    }

    fn parse_texture(&mut self, line: &str, index: usize) {
        let mut iter = line.split_whitespace();
        iter.next();
        let x = iter.next().unwrap().parse::<f32>().unwrap();
        let y = iter.next().unwrap().parse::<f32>().unwrap();
        if let Some(z) = iter.next() {
            self.vertices[index].tex_coords = (x, y, z.parse::<f32>().unwrap());
        } else {
            self.vertices[index].tex_coords = (x, y, 0.0);
        }
    }

    fn parse_triangle(&mut self, line: &str, mat_to_use: usize) {
        let mut iter = line.split_whitespace();
        iter.next();
        let first = iter.next().unwrap().split("/").collect::<Vec::<_> >()[0].parse::<u16>().unwrap();
        let mut second = iter.next().unwrap().split("/").collect::<Vec::<_> >()[0].parse::<u16>().unwrap();
        for it in iter {
            let third = it.split("/").collect::<Vec::<_> >()[0].parse::<u16>().unwrap();
            self.indices.push(first - 1);
            self.indices.push(second - 1);
            self.indices.push(third - 1);
            self.link_material.push(mat_to_use);

            second = third;
        }
    }

    fn load_material(&mut self, line: &str, path: &str) {
        let mut iter = line.split_whitespace();
        iter.next();
        let file = iter.next().unwrap().to_string();
        let mut base_path = path.split("/").collect::<Vec<&str>>();
        base_path.pop();
        base_path.push(&file);
        let path = base_path.join("/");
        println!("Loading material from {}", path);
        self.materials = Material::load_materials(&path)
    }

    pub fn center_object(&mut self) -> Self {
        let mut min = (f32::MAX, f32::MAX, f32::MAX);
        let mut max = (f32::MIN, f32::MIN, f32::MIN);
        for vertex in &self.vertices {
            min.0 = min.0.min(vertex.position.0);
            min.1 = min.1.min(vertex.position.1);
            min.2 = min.2.min(vertex.position.2);
            max.0 = max.0.max(vertex.position.0);
            max.1 = max.1.max(vertex.position.1);
            max.2 = max.2.max(vertex.position.2);
        }
        let center = ((max.0 + min.0) / 2.0, (max.1 + min.1) / 2.0, (max.2 + min.2) / 2.0);
        for vertex in &mut self.vertices {
            vertex.position.0 -= center.0;
            vertex.position.1 -= center.1;
            vertex.position.2 -= center.2;
        }
        self.clone()
    }

    pub fn load_triangle_normal(&mut self) {
        for i in 0..self.indices.len() / 3 {
            let index = i * 3;
            let first = self.vertices[self.indices[index] as usize].position;
            let second = self.vertices[self.indices[index + 1] as usize].position;
            let third = self.vertices[self.indices[index + 2] as usize].position;
            let normal = Vector::cross_product(&Vector::from(&[second.0 - first.0, second.1 - first.1, second.2 - first.2]),
                                               &Vector::from(&[third.0 - first.0, third.1 - first.1, third.2 - first.2]));
            let normal = normal.rnormalize();
            // self.normals.push(Normal { normal: (normal[0], normal[1], normal[2]) });
            // self.normals.push(Normal { normal: (normal[0], normal[1], normal[2]) });
            // self.normals.push(Normal { normal: (normal[0], normal[1], normal[2]) });

            let face1 = Vector::cross_product(&normal, &Vector::from(&[1.0, 0.0, 0.0f32])).norm();
            let face2 = Vector::cross_product(&normal, &Vector::from(&[0.0, 1.0, 0.0f32])).norm();
            let face3 = Vector::cross_product(&normal, &Vector::from(&[0.0, 0.0, 1.0f32])).norm();

            if face1 > face2 && face1 > face3 {
                self.triangle_normals.push(Face {face: 0});
            } else if face2 > face1 && face2 > face3 {
                self.triangle_normals.push(Face {face: 1});
            } else {
                self.triangle_normals.push(Face {face: 2});
            }
        }
    }

    pub fn from_path(path: &str) -> Self {
        println!("Loading {}", path);
        let mut obj = Self::default();
        let mut texture_count: usize = 0;

        let mut mat_to_use: usize = 0;
        let obj_file = fs::read_to_string(path).expect("Unable to read file");

        for line in obj_file.lines() {
            if let Some(key) = line.split_whitespace().next() {
                match key {
                    "v" => {
                        obj.parse_vertex(line);
                    },
                    "vn" => {
                        obj.parse_normal(line);
                    },
                    "vt" => {
                        obj.parse_texture(line, texture_count);
                        texture_count += 1;
                    },
                    "f" => {
                        obj.parse_triangle(line, mat_to_use);
                    },
                    "mtllib" => {
                        obj.load_material(line, path)
                    },
                    "o" => {
                        obj.name = line.split_whitespace().nth(1).unwrap().to_string();
                    },
                    "usemtl" => {
                        mat_to_use = get_index_by_name(&obj.materials, &line.split_whitespace().nth(1).unwrap().to_string());
                    }
                    _ => {}
                }
            }
        }
        if obj.materials.len() == 0 {
            obj.materials.push(Material::default());
        }
        if obj.normals.len() == 0 {
            obj.normals = obj.vertices.iter().map(|vert| Normal { normal: vert.position }).collect();
        }

        // obj.load_triangle_normal();

        obj        
    }
}