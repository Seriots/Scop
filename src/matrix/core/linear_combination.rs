#![allow(dead_code)]

use crate::matrix::{core::Vector, utils::{NumberUtils, Fma}};

pub fn linear_combination<K: Clone + Default + NumberUtils + Fma>(u: &[Vector<K>], coefs: &[K]) -> Vector<K> {
    let mut mu = Vector::from_vec(vec![K::default(); u[0].size()]);

    for i in 0..u.len() {
        for j in 0..u[0].size() {
            mu[j].sfma(u[i][j].clone(), coefs[i].clone());
        }  
    } 
    return mu;
}
