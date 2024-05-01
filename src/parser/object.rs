
use crate::teapot::{Normal, Vertex};

pub struct Object {
    pub vertices: Vec<Vertex>,
    pub normals: Vec<Normal>,
    pub indices: Vec<u16>
}

impl Object {
    pub fn new(vertices: Vec<Vertex>, normals: Vec<Normal>, indices: Vec<u16>) -> Self {
        Self {vertices, normals, indices}
    }
}