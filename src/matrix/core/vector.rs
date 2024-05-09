#![allow(dead_code)]

use std::{cmp::{max, Ordering}, default, fmt::Display, ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Range, Sub, SubAssign}, process::Output};

use crate::matrix::{utils::NumberUtils, Matrix};
use crate::matrix::utils::Fma;
use crate::matrix::core::linear_interpolation::Lerp;

#[derive(Debug, Clone, Default)]
pub struct Vector<K> {
    pub vector: Vec<K>,
}

impl<K: Clone + Default + NumberUtils + Fma + Sub<Output = K>> PartialEq for Vector<K> {
    fn eq(&self, other: &Self) -> bool {
        if self.size() != other.size() {
            return false;
        }
        for i in 0..self.size() {
            if !(self[i].clone() - other[i].clone()).approx_zero() {
                return false;
            }
        }
        return true;
    }
}

impl<K: Clone + Default + NumberUtils + Fma> Vector<K> {
    pub fn from(vector: &[K]) -> Self {
        Self { vector: vector.to_vec() }
    }

    pub fn from_vec(vector: Vec<K>) -> Self {
        Self { vector }
    }
    pub fn size(&self) -> usize {
        return self.vector.len();
    }

    pub fn to_matrix(&self, shape: (usize, usize)) -> Matrix<K> {
        let mut matrix: Vec<Vec<K>> = Vec::new();
        
        if shape.0 == 0 || shape.1 == 0 {
            panic!("Shape can't be 0")
        }
        if shape.0 * shape.1 != self.size() {
           panic!("This vector can't be converted in this shape") 
        }
        for i in 0..shape.1 {
            matrix.push(Vec::new());
            matrix[i].append(&mut self[(i*shape.1)..(i*shape.1 + shape.0)].to_vec());
        }
        return Matrix::from_vec(matrix)
    }

    pub fn to_list_3(&self) -> [K; 3] {
        if self.size() != 3 {
            panic!("Size must be 3")
        }
        return [self[0].clone(), self[1].clone(), self[2].clone()];
    }
}


impl<K: Clone + Default + NumberUtils + Fma + AddAssign + DivAssign + SubAssign + MulAssign + Mul<Output = K> + Sub<Output = K> + Add<Output = K> + Div<Output = K> + Neg<Output = K> + PartialEq + PartialOrd> Vector<K>
{
    pub fn add(&mut self, v: &Vector<K>) {
        if self.size() != v.size() {
            panic!("Size are different")
        }
        for i in 0..self.size() {
            self[i] += v[i].clone();
        }
    }

    pub fn sub(&mut self, v: &Vector<K>) {
        if self.size() != v.size() {
            panic!("Size are different")
        }
        for i in 0..self.size() {
            self[i] -= v[i].clone();
        }
    }

    pub fn scl(&mut self, a: K) {
        for i in 0..self.size() {
            self[i] *= a.clone(); 
        }
    }

    pub fn add_new(&self, a: K) -> Vector<K> {
        let mut new = self.clone();
        for i in 0..new.size() {
            new[i] += a.clone(); 
        }
        new
    }
    
    pub fn scl_new(&self, a: K) -> Vector<K> {
        let mut new = self.clone();
        for i in 0..new.size() {
            new[i] *= a.clone(); 
        }
        new
    }

    pub fn dot(&self, v: Self) -> K {
        let mut result: K = K::default();

        for i in 0..self.size() {
            result.sfma(self[i].clone(), v[i].clone());
        }
        return result;
    }

    //Taxicab norm or Manhattan norm (||v||1)
    pub fn norm_1(&self) -> K {
        let mut res = K::default();
        
        for i in 0..self.size() {
            let v: K = self[i].clone();
            res += v.absolute();
        }

        return res;
    }

    //Euclidean norm (||v||2)
    pub fn norm(&self) -> K {
        let mut res = K::default();

        for i in 0..self.size() {
            res.sfma(self[i].clone(), self[i].clone());
        }
        return res.squarert();
    }

    // Supremum norm (||v||∞)
    pub fn norm_inf(&self) -> K {

        let mut res = K::default();
        
        for i in 0..self.size() {
            let v: K = self[i].clone();
            res = if res.partial_cmp(&v.absolute()) == Some(Ordering::Less) {
                v.absolute()
            } else {
                res
            };
        }

        return res;
    }

    pub fn normalize(&mut self) {
        let norm = self.norm();
        for i in 0..self.size() {
            self[i] /= norm.clone();
        }
    }

    pub fn rnormalize(&self) -> Self {
        let mut new = self.clone();
        let norm = new.norm();
        for i in 0..new.size() {
            new[i] /= norm.clone();
        }
        new 
    }

    pub fn angle_cos(u: &Vector<K>, v: &Vector<K>) -> K
    {
        let dot: K = u.clone().dot(v.clone());
        let norm: K = u.clone().norm() * v.clone().norm();
        
        return  dot / norm;
    }

    pub fn cross_product(u: &Vector<K>, v: &Vector<K>) -> Vector<K> {

        return Vector::from(&[u[1].clone() * v[2].clone() - u[2].clone() * v[1].clone(),
                                        u[2].clone() * v[0].clone() - u[0].clone() * v[2].clone(),
                                        u[0].clone() * v[1].clone() - u[1].clone() * v[0].clone()]);
    }

}

impl<K: Clone + Default + NumberUtils + Fma + Sub<Output = K> + Add<Output = K> + Mul<Output = K> + From<f32>> Lerp for Vector<K> {
    fn lerp(self, v: Self, t: f32) -> Self {
       let mut res = self.clone();

        for i in 0..self.size() {
            //(v - u) * t + u
            res[i] = Fma::fma(v[i].clone() - self[i].clone(), t.into(), self[i].clone()); 
        }
        return res;
    }
}

// ------------------------------------------------------------------------- //
/// Implementing accesor => [] operator

impl<K> Index<usize> for Vector<K> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vector[index]
    }
}

impl<K> IndexMut<usize> for Vector<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vector[index]
    }
}

impl<K> Index<Range<usize>> for Vector<K> {
    type Output = [K];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.vector[range]
    }
}

impl<K> IndexMut<Range<usize>> for Vector<K> {
    fn index_mut(&mut self, range: Range<usize>) -> &mut Self::Output {
        &mut self.vector[range]
    }
}

// ------------------------------------------------------------------------- //

/// Implementing Display for Vector
impl<K: Clone + Default + NumberUtils + Fma + Display> Display for Vector<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut all: String = String::new();
        let mut max_size = 0;
        for i in 0..self.size() {
            if  self[i].to_string().len() > max_size {
                max_size = self[i].to_string().len()
            }
        }
        for i in 0..self.size() {
            all += &format!("[{:>max$}]", self[i], max=max_size);
            if i != self.size() -1 {
                all += "\n";
            }

        }
        write!(f, "{}", all)
    }
}

