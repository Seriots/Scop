
use std::fs;

use glium::implement_vertex;

use crate::Material;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: (f32, f32, f32),
    pub color: (f32, f32, f32)
}

implement_vertex!(Vertex, position, color);

#[derive(Copy, Clone)]
pub struct Normal {
    pub normal: (f32, f32, f32)
}

implement_vertex!(Normal, normal);

#[derive(Default, Clone)]
pub struct Object {
    pub name: String,
    pub vertices: Vec<Vertex>,
    pub normals: Vec<Normal>,
    pub indices: Vec<u16>,
    pub material_path: String,
    pub materials: Vec<Material>,
    pub link_material: Vec<String>,
    pub color: Vec<Vertex>,
}

impl Object {
    pub fn new(vertices: Vec<Vertex>, normals: Vec<Normal>, indices: Vec<u16>) -> Self {
        Self {
                name: String::default(),
                vertices,
                normals,
                indices,
                material_path: String::default(),
                materials: Default::default(),
                link_material: Default::default(),
                color: Default::default()
            }
    }

    fn parse_vertex(&mut self, line: &str) {
        let mut iter = line.split_whitespace();
        iter.next();
        let x = iter.next().unwrap().parse::<f32>().unwrap();
        let y = iter.next().unwrap().parse::<f32>().unwrap();
        let z = iter.next().unwrap().parse::<f32>().unwrap();
        let random_value = rand::random::<f32>();
        self.vertices.push(Vertex { position: (x, y, z), color: (random_value, random_value, random_value)})
    }

    fn parse_normal(&mut self, line: &str) {
        let mut iter = line.split_whitespace();
        iter.next();
        let x = iter.next().unwrap().parse::<f32>().unwrap();
        let y = iter.next().unwrap().parse::<f32>().unwrap();
        let z = iter.next().unwrap().parse::<f32>().unwrap();
        self.normals.push(Normal { normal: (x, y, z) })
    }

    fn parse_triangle(&mut self, line: &str, mat_to_use: &str) {
        let mut iter = line.split_whitespace();
        iter.next();
        let first = iter.next().unwrap().parse::<u16>().unwrap();
        let mut second = iter.next().unwrap().parse::<u16>().unwrap();
        for it in iter {
            let third = it.parse::<u16>().unwrap();
            self.indices.push(first - 1);
            self.indices.push(second - 1);
            self.indices.push(third - 1);
            self.link_material.push(mat_to_use.to_string());

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


    pub fn from_path(path: &str) -> Self {
        println!("Loading {}", path);
        let mut obj = Self::default();

        let mut mat_to_use = String::from("default");
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
                    "f" => {
                        obj.parse_triangle(line, &mat_to_use);
                    },
                    "mtllib" => {
                        obj.load_material(line, path)
                    },
                    "o" => {
                        obj.name = line.split_whitespace().nth(1).unwrap().to_string();
                    },
                    "usemtl" => {
                        mat_to_use = line.split_whitespace().nth(1).unwrap().to_string();
                    }
                    _ => {}
                }
            }
            //let key = line.split_whitespace().next().unwrap();
        }
        obj        
    }
}