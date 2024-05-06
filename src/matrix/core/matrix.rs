#![allow(dead_code)]
use std::{any::TypeId, default, fmt::{Debug, Display}, ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Range, Sub, SubAssign}};

use crate::matrix::utils::NumberUtils;
use crate::matrix::Vector;
use crate::matrix::utils::Fma;
use std::cmp::min;

use crate::matrix::core::linear_interpolation::Lerp;
#[derive(Debug)]
pub enum MatrixError {
    SingularMatrix
}

impl Display for MatrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatrixError::SingularMatrix => write!(f, "This matrix is singular")
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Matrix<K> {
    pub matrix: Vec<Vec<K>>,
}

impl<K: Clone + Default + NumberUtils + Fma + Sub<Output = K>> PartialEq for Matrix<K> {
    fn eq(&self, other: &Self) -> bool {
        if self.shape() != other.shape() {
            return false;
        }
        for i in 0..self.shape().1 {
            for j in 0..self.shape().0 {
                if !(self[i][j].clone() - other[i][j].clone()).approx_zero() {
                    return false;
                }
            }
        }
        return true;
    }
}

impl<K: Clone + Default + NumberUtils + Fma> Matrix<K> {
    pub fn from(matrix: &[&[K]]) -> Self {
        let mut new_matrix: Vec<Vec<K>> = Vec::new();

        for i in 0..matrix.len() {
            if matrix[i].len() != matrix[0].len() {
                panic!("All rows must have the same length");
            }
            new_matrix.push(matrix[i].to_vec());
        }
        Self { matrix: new_matrix }
    }

    pub fn from_vec(matrix: Vec<Vec<K>>) -> Self {
        for i in 0..matrix.len() {
            if matrix[i].len() != matrix[0].len() {
                panic!("All rows must have the same length");
            }
        }
        Self { matrix }
    }

    pub fn from_vector(vectors: &[Vector<K>]) -> Self {
        let mut new = Vec::new();
        for vector in vectors {
            let mut row = Vec::new();
            for i in 0..vector.size() {
                row.push(vector[i].clone());
            }
            new.push(row)
        }
        Self { matrix: new }
    }

    pub fn shape(&self) -> (usize, usize) {
        return (self[0].len(), self.matrix.len())
    }

    pub fn is_square(&self) -> bool {
        return self[0].len() == self.matrix.len()
    }

    pub fn to_vector(&self) -> Vector<K> {
        let mut vector: Vec<K> = Vec::new();

        for i in 0..self.matrix.len() {
            vector.append(&mut self[i].clone());
        }
        return Vector::from_vec(vector);
    }

    pub fn to_list_4(&self) -> [[K; 4]; 4] {
        if self.shape() != (4, 4) {
            panic!("The matrix must be a 4x4 matrix")
        }
        
        [
            [self[0][0].clone(), self[0][1].clone(), self[0][2].clone(), self[0][3].clone()],
            [self[1][0].clone(), self[1][1].clone(), self[1][2].clone(), self[1][3].clone()],
            [self[2][0].clone(), self[2][1].clone(), self[2][2].clone(), self[2][3].clone()],
            [self[3][0].clone(), self[3][1].clone(), self[3][2].clone(), self[3][3].clone()],
        ]
    }
}

impl<K: Debug + Display + NumberUtils + Clone + Default + Fma + AddAssign + Add<Output = K> + SubAssign + Sub<Output = K> + MulAssign + DivAssign + Mul<Output = K> + Div<Output = K> + Neg<Output = K> + PartialEq> Matrix<K> {
    pub fn add(&mut self, v: &Matrix<K>) {
        if self.shape() != v.shape() {
            panic!("Size are different")
        }
        for i in 0..self.shape().0 {
            for j in 0..self.shape().1 {
                self[i][j] += v[i][j].clone();
            }
        }
    }

    pub fn sub(&mut self, v: &Matrix<K>) {
        if self.shape() != v.shape() {
            panic!("Size are different")
        }
        for i in 0..self.shape().0 {
            for j in 0..self.shape().1 {
                self[i][j] -= v[i][j].clone();
            }
        }
    }

    pub fn scl(&mut self, a: K) {
        for i in 0..self.shape().0 {
            for j in 0..self.shape().1 {
                self[i][j] *= a.clone();
            }
        }
    }

    pub fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        let base_shape = self.shape();

        if vec.size() != base_shape.0 {
            panic!("Vector must have the same size than the number of columns")
        }

        let mut new = Vector::from_vec(vec![K::default(); base_shape.1]);
        for i in 0..base_shape.0 {
            for j in 0..base_shape.1 {
                new[j].sfma(vec[i].clone(), self[j][i].clone());
            }
        }

        return new;
    }

    pub fn mul_mat(&self, mat: &Matrix<K>) -> Matrix<K> {
        let base_shape = self.shape();
        let mut new: Matrix<K> = Matrix::from_vec(vec![vec![K::default(); mat.shape().0]; base_shape.1]);
        
        if mat.shape().1 != base_shape.0 {
            panic!("Argument must have the same amount of rows than the amout of columns")
        }
        for k in 0..mat.shape().0 {
            for i in 0..base_shape.0 {
                for j in 0..base_shape.1 {
                   new[j][k].sfma(mat[i][k].clone(), self[j][i].clone()); 
                }
            }
        }
        return new;
    }

    pub fn trace(&self) -> K {
        let mut tr: K = K::default();
        if !self.is_square() {
            panic!("This matrix is not a square")
        }
        for i in 0..self.shape().0 {
            tr += self[i][i].clone();
        }
        return tr;
    }

    pub fn transpose(&self) -> Matrix<K> {
        
        let mut new: Matrix<K> = Matrix::from_vec(vec![vec![K::default(); self.shape().1]; self.shape().0]);

        for i in 0..self.shape().0 {
            for j in 0..self.shape().1 {
                new[i][j] = self[j][i].clone();
            }
        }
        return new;
    }

    fn scale_row(&mut self, row_index: usize, scaler: K) {
        for i in 0..self.shape().0 {
            self[row_index][i] /= scaler.clone();
        }
    }

    fn add_row(&mut self, row_index: usize, row_adder: Vec<K>, scaler: K) {
        for i in 0..self.shape().0 {
            self[row_index][i] += scaler.clone() * row_adder[i].clone();
        }
    }

    fn swap_row(&mut self, row_one: usize, row_two: usize) {
        let tmp = self[row_one].clone();
        self[row_one] = self[row_two].clone();
        self[row_two] = tmp;
    }

    pub fn row_echelon(&self) -> Matrix<K> {
        let mut new = self.clone();
        let cols = self.shape().0;
        let rows = self.shape().1;

        let mut pivot_col: usize = 0;

        for row in 0..rows {
            for col in pivot_col..cols {
                let mut swapped = false;
                if new[row][col] == K::default() {
                    for row_to_swap in (row + 1)..rows {
                        if new[row_to_swap][col] != K::default() {
                            new.swap_row(row, row_to_swap);
                            swapped = true;
                            break;
                        }
                    }
                }
                if swapped || new[row][col] != K::default() {
                    pivot_col = col;
                    new.scale_row(row, new[row][col].clone());
                    for row_to_zero in 0..rows {
                        if row_to_zero == row || new[row_to_zero][col] == K::default() {
                            continue ;
                        }
                        new.add_row(row_to_zero,
                                    new[row].clone(),
                                    -(new[row_to_zero][col].clone() / new[row][col].clone())
                                );
                    }
                    break ;
                }
            }
        }
        return new;
    }

    fn determinant_2d(mat: &Matrix<K>) -> K {
        return Fma::fma(mat[0][0].clone(),  mat[1][1].clone(), -mat[0][1].clone() * mat[1][0].clone());
    }

    fn determinant_3d(mat: &Matrix<K>) -> K {
        return Fma::fma(mat[0][0].clone(), Matrix::determinant_2d(&Matrix::from_vec(vec![mat[1][1..=2].to_vec(), mat[2][1..=2].to_vec()])), 
               Fma::fma(-mat[0][1].clone(), Matrix::determinant_2d(&Matrix::from_vec(vec![vec![mat[1][0].clone(), mat[1][2].clone()], vec![mat[2][0].clone(), mat[2][2].clone()]])),
                        mat[0][2].clone() * Matrix::determinant_2d(&Matrix::from_vec(vec![mat[1][0..=1].to_vec(), mat[2][0..=1].to_vec()])))
                    );
    }

    fn determinant_4d(mat: &Matrix<K>) -> K {
        return Fma::fma(mat[0][0].clone(), Matrix::determinant_3d(&Matrix::from_vec(vec![mat[1][1..=3].to_vec(), mat[2][1..=3].to_vec(), mat[3][1..=3].to_vec()])),
               Fma::fma(-mat[0][1].clone(), Matrix::determinant_3d(&Matrix::from_vec(vec![vec![mat[1][0].clone(), mat[1][2].clone(), mat[1][3].clone()], vec![mat[2][0].clone(), mat[2][2].clone(), mat[2][3].clone()], vec![mat[3][0].clone(), mat[3][2].clone(), mat[3][3].clone()]])),
               Fma::fma(mat[0][2].clone(), Matrix::determinant_3d(&Matrix::from_vec(vec![vec![mat[1][0].clone(), mat[1][1].clone(), mat[1][3].clone()], vec![mat[2][0].clone(), mat[2][1].clone(), mat[2][3].clone()], vec![mat[3][0].clone(), mat[3][1].clone(), mat[3][3].clone()]])),
                        -mat[0][3].clone() * Matrix::determinant_3d(&Matrix::from_vec(vec![mat[1][0..=2].to_vec(), mat[2][0..=2].to_vec(), mat[3][0..=2].to_vec()])))
                        )
                    );
    }

    pub fn determinant(&self) -> K {
        if !self.is_square() {
            panic!("This matrix is not a square")
        }
        let shape = self.shape().0;
        match shape {
            1 => return self[0][0].clone(),
            2 => return Matrix::determinant_2d(&self),
            3 => return Matrix::determinant_3d(&self),
            4 => return Matrix::determinant_4d(&self),
            _ => panic!("The matrix size is greater than 4")
        }
    }

    fn get_identity_matrix(&self) -> Matrix<K> {
        if !self.is_square() {
            panic!("The matrix is not a square")
        }
        let mut new = Matrix::from_vec(vec![vec![K::default(); self.shape().0]; self.shape().1]);
        for i in 0..self.shape().0 {
            new[i][i] = K::one();
        }
        return new;
    }

    pub fn inverse(&self) -> Result<Matrix<K>, MatrixError> {
        let mut new = self.clone();
        let mut identity = self.get_identity_matrix();
        let cols = self.shape().0;
        let rows = self.shape().1;

        if !self.is_square() {
            panic!("The matrix is not a square")
        }

        if self.determinant() == K::default() {
            return Err(MatrixError::SingularMatrix)
        }

        let mut pivot_col: usize = 0;

        for row in 0..rows {
            for col in pivot_col..cols {
                let mut swapped = false;
                if new[row][col] == K::default() {
                    for row_to_swap in (row + 1)..rows {
                        if new[row_to_swap][col] != K::default() {
                            identity.swap_row(row, row_to_swap);
                            new.swap_row(row, row_to_swap);
                            swapped = true;
                            break;
                        }
                    }
                }
                if swapped || new[row][col] != K::default() {
                    pivot_col = col;
                    identity.scale_row(row, new[row][col].clone());
                    new.scale_row(row, new[row][col].clone());
                    for row_to_zero in 0..rows {
                        if row_to_zero == row || new[row_to_zero][col] == K::default() {
                            continue ;
                        }
                        identity.add_row(row_to_zero,
                                    identity[row].clone(),
                                    -(new[row_to_zero][col].clone() / new[row][col].clone())
                                );
                        new.add_row(row_to_zero,
                                    new[row].clone(),
                                    -(new[row_to_zero][col].clone() / new[row][col].clone())
                                );
                    }
                    break ;
                }
            }
        }
        return Ok(identity);
    }

    pub fn rank(&self) -> usize {
        let mut new = self.clone();
        let cols = self.shape().0;
        let rows = self.shape().1;

        let mut pivot_col: usize = 0;
        let mut rank = 0;

        for row in 0..rows {
            for col in pivot_col..cols {
                let mut swapped = false;
                if new[row][col] == K::default() {
                    for row_to_swap in (row + 1)..rows {
                        if new[row_to_swap][col] != K::default() {
                            new.swap_row(row, row_to_swap);
                            swapped = true;
                            break;
                        }
                    }
                }
                if swapped || new[row][col] != K::default() {
                    pivot_col = col;
                    new.scale_row(row, new[row][col].clone());
                    rank += 1;
                    for row_to_zero in 0..rows {
                        if row_to_zero == row || new[row_to_zero][col] == K::default() {
                            continue ;
                        }
                        new.add_row(row_to_zero,
                                    new[row].clone(),
                                    -(new[row_to_zero][col].clone() / new[row][col].clone())
                                );
                    }
                    break ;
                }
            }
        }

        return rank;
    }
}


impl<K: Clone + Default + NumberUtils + Fma + Sub<Output = K> + From<f32>> Lerp for Matrix<K> {
    fn lerp(self, v: Self, t: f32) -> Self {
       let mut res = self.clone();
       let shape = self.shape();

        for i in 0..shape.0 {
            for j in 0..shape.1 {
                res[i][j] = Fma::fma(v[i][j].clone() - self[i][j].clone(), t.into(), self[i][j].clone()); 
            }
        }
        return res;
    }
}

// ------------------------------------------------------------------------- //
/// Implementing Index and IndexMut for Matrix => accssor []

impl<K> Index<usize> for Matrix<K> {
    type Output = Vec<K>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
}

impl<K> IndexMut<usize> for Matrix<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.matrix[index]
    }
}

impl<K> Index<(usize, usize)> for Matrix<K> {
    type Output = K;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.matrix[index.0][index.1]
    }
}

impl<K> IndexMut<(usize, usize)> for Matrix<K> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.matrix[index.0][index.1]
    }
}

impl<K> Index<Range<usize>> for Matrix<K> {
    type Output = [Vec<K>];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.matrix[range]
    }
}

impl<K> IndexMut<Range<usize>> for Matrix<K> {
    fn index_mut(&mut self, range: Range<usize>) -> &mut Self::Output {
        &mut self.matrix[range]
    }
}

impl<K> Index<(Range<usize>, Range<usize>)> for Matrix<K> {
    type Output = [Vec<K>];

    fn index(&self, range:(Range<usize>, Range<usize>)) -> &Self::Output {
        &self.matrix[range.0][range.1]
    }
}

impl<K> IndexMut<(Range<usize>, Range<usize>)> for Matrix<K> {
    fn index_mut(&mut self, range: (Range<usize>, Range<usize>)) -> &mut Self::Output {
        &mut self.matrix[range.0][range.1]
    }
}


impl Mul for Matrix<f32> {
    type Output = Matrix<f32>;

    fn mul(self, v: Self) -> Self::Output {
        return self.mul_mat(&v);
    }
}

// ------------------------------------------------------------------------- //
/// Implementing Display for matrix

impl<K: Clone + Default + NumberUtils + Fma + Display> Display for Matrix<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let mut all_rows: String = String::new();
        let mut max_size: Vec<usize> = Vec::new();
        let shape = self.shape();

        for i in 0..shape.1 {
            for j in 0..shape.0 {
                if i == 0 { max_size.push(0); }
                if let Some(prec) = f.precision() {
                    if min(self[i][j].to_string().len(), prec + 3) > max_size[j] {
                        max_size[j] = min(self[i][j].to_string().len(), prec + 3);
                    }
                }
                else {
                    if self[i][j].to_string().len() > max_size[j] {
                        max_size[j] = self[i][j].to_string().len();
                    }
                }
            }
        }

        for i in 0..shape.1 {
            all_rows += "[ ";
            for j in 0..shape.0 {
                if let Some(prec) = f.precision() {
                    all_rows += &format!("{:>max$.prec$} ", &self[i][j], max=max_size[j], prec=prec);
                }
                else {
                    all_rows += &format!("{:>max$} ", &self[i][j], max=max_size[j]);
                }
            }
            all_rows += "]";
            if i != shape.1 - 1 {
                all_rows += "\n";
            }
        } 
        write!(f, "{}", all_rows)
    }
}