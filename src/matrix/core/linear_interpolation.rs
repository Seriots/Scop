#![allow(dead_code)]

use num_traits::{ops::mul_add::MulAdd, Num, PrimInt};

use std::ops::Sub;

use crate::matrix::utils::Fma;

pub trait Lerp: Sized {
    fn lerp(self, v: Self, t: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(self, v: f32, t: f32) -> f32 {
        return Fma::fma(v.clone() - self.clone(), t, self.clone());
    }
}

impl Lerp for f64 {
    fn lerp(self, v: f64, t: f32) -> f64 {
        return Fma::fma((v.clone() - self.clone()) as f32, t, self.clone() as f32) as f64;
    }
}

impl Lerp for i8 {
    fn lerp(self, v: i8, t: f32) -> i8 {
        return Fma::fma((v.clone() - self.clone()) as f32, t, self.clone() as f32) as i8;
    }
}

impl Lerp for i16 {
    fn lerp(self, v: i16, t: f32) -> i16 {
        return Fma::fma((v.clone() - self.clone()) as f32, t, self.clone() as f32) as i16;
    }
}

impl Lerp for i32 {
    fn lerp(self, v: i32, t: f32) -> i32 {
        return Fma::fma((v.clone() - self.clone()) as f32, t, self.clone() as f32) as i32;
    }
}

impl Lerp for i64 {
    fn lerp(self, v: i64, t: f32) -> i64 {
        return Fma::fma((v.clone() - self.clone()) as f32, t, self.clone() as f32) as i64;
    }
}

impl Lerp for i128 {
    fn lerp(self, v: i128, t: f32) -> i128 {
        return Fma::fma((v.clone() - self.clone()) as f32, t, self.clone() as f32) as i128;
    }
}

impl Lerp for isize {
    fn lerp(self, v: isize, t: f32) -> isize {
        return Fma::fma((v.clone() - self.clone()) as f32, t, self.clone() as f32) as isize;
    }
}

impl Lerp for u8 {
    fn lerp(self, v: u8, t: f32) -> u8 {
        return Fma::fma((v.clone() - self.clone()) as f32, t, self.clone() as f32) as u8;
    }
}

impl Lerp for u16 {
    fn lerp(self, v: u16, t: f32) -> u16 {
        return Fma::fma((v.clone() - self.clone()) as f32, t, self.clone() as f32) as u16;
    }
}

impl Lerp for u32 {
    fn lerp(self, v: u32, t: f32) -> u32 {
        return Fma::fma((v.clone() - self.clone()) as f32, t, self.clone() as f32) as u32;
    }
}

impl Lerp for u64 {
    fn lerp(self, v: u64, t: f32) -> u64 {
        return Fma::fma((v.clone() - self.clone()) as f32, t, self.clone() as f32) as u64;
    }
}

impl Lerp for u128 {
    fn lerp(self, v: u128, t: f32) -> u128 {
        return Fma::fma((v.clone() - self.clone()) as f32, t, self.clone() as f32) as u128;
    }
}

impl Lerp for usize {
    fn lerp(self, v: usize, t: f32) -> usize {
        return Fma::fma((v.clone() - self.clone()) as f32, t, self.clone() as f32) as usize;
    }
}

pub fn lerp<V: Lerp>(u: V, v: V, t: f32) -> V {
    return u.lerp(v, t);
}
