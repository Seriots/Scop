use std::fs;


#[derive(Clone, Default, Debug)]
pub struct Material {
    pub name: String,
    pub specular_coef: f32,
    pub ambient_color: [f32; 3],
    pub diffuse_color: [f32; 3],
    pub specular_color: [f32; 3],
    pub emissive_color: [f32; 3],
    pub refraction: f32,
    pub alpha: f32,
    pub illum_type: u16,
}

impl Material {
    pub fn basics() -> Self {
        Self {
            alpha: 1.0,
            ambient_color: [1.0, 0.0, 0.0],
            specular_coef: 1.4,
            diffuse_color: [0.64, 0.64, 0.64],
            specular_color: [0.5, 0.5, 0.5],
            refraction: 1.0,
            emissive_color: [0.0, 0.0, 0.0],
            name: String::from("default"),
            ..Default::default()
        }
    } 

    pub fn load_materials(path: &str) -> Vec<Material> {
        let mut new: Vec<Material> = Vec::new();
        let material_file = fs::read_to_string(path).unwrap();
        let mut current_material = Self::basics();
        let mut first = true;

        for line in material_file.lines() {
            let mut it = line.split_whitespace();
            if let Some(key) = it.next() {
                match key {
                    "newmtl" => {
                        if first {
                            first = false;
                        } else {
                            new.push(current_material);
                            current_material = Self::default();
                        }
                        current_material.name = it.next().unwrap().to_string();
                    },
                    "Ns" => {
                        current_material.specular_coef = it.next().unwrap().parse::<f32>().unwrap();
                    },
                    "Ka" => {
                        current_material.ambient_color = [
                            it.next().unwrap().parse::<f32>().unwrap(),
                            it.next().unwrap().parse::<f32>().unwrap(),
                            it.next().unwrap().parse::<f32>().unwrap(),
                        ]
                    },
                    "Kd" => {
                        current_material.diffuse_color = [
                            it.next().unwrap().parse::<f32>().unwrap(),
                            it.next().unwrap().parse::<f32>().unwrap(),
                            it.next().unwrap().parse::<f32>().unwrap(),
                        ]
                    },
                    "Ks" => {
                        current_material.specular_color = [
                            it.next().unwrap().parse::<f32>().unwrap(),
                            it.next().unwrap().parse::<f32>().unwrap(),
                            it.next().unwrap().parse::<f32>().unwrap(),
                        ]
                    },
                    "Ke" => {
                        current_material.emissive_color = [
                            it.next().unwrap().parse::<f32>().unwrap(),
                            it.next().unwrap().parse::<f32>().unwrap(),
                            it.next().unwrap().parse::<f32>().unwrap(),
                        ]
                    },
                    "Ni" => {
                        current_material.refraction = it.next().unwrap().parse::<f32>().unwrap();
                    },
                    "d" | "Tr" => {
                        current_material.alpha = it.next().unwrap().parse::<f32>().unwrap();
                    },
                    "illum" => {
                        current_material.illum_type = it.next().unwrap().parse::<u16>().unwrap();

                    },
                    _ => {}
                }
            }
            //let key = line.split_whitespace().next().unwrap();
        }
        new.push(current_material);
        new
        
    }
}

pub fn get_index_by_name(mat_vec: &Vec<Material>, name: &String) -> usize {
    for (i, elem) in mat_vec.iter().enumerate() {
        if &elem.name == name {
            return i
        }
    }
    return 0
}