use crate::scalar::base_float::BaseFloat;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub, SubAssign};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Index2D {
    pub row: usize,
    pub col: usize,
}

impl Index2D {
    pub fn to_1d(&self, col_size: usize) -> usize {
        self.row * col_size + self.col
    }
}

impl From<(usize, usize)> for Index2D {
    fn from(value: (usize, usize)) -> Self {
        Index2D {
            row: value.0,
            col: value.1,
        }
    }
}

pub trait MatrixShape {
    fn row_size(&self) -> usize;
    fn col_size(&self) -> usize;
    fn square_dimention(&self) -> Option<usize> {
        if self.row_size() == self.col_size() {
            Some(self.row_size())
        } else {
            None
        }
    }
    fn shape(&self) -> Index2D {
        Index2D {
            row: self.row_size(),
            col: self.col_size(),
        }
    }
}

pub trait MatrixOps<T: BaseFloat>:
    MatrixShape + Index<Index2D, Output = T> + IndexMut<Index2D> + Clone
{
    fn default_with_shape(shape: Index2D) -> Self;

    fn shape_eq(&self, rhs: &impl MatrixOps<T>) -> bool {
        self.shape() == rhs.shape()
    }

    fn add(&self, rhs: &impl MatrixOps<T>) -> Self {
        assert!(self.shape_eq(rhs));
        let mut mat = Self::default_with_shape(self.shape());
        for row in 0..self.row_size() {
            for col in 0..self.col_size() {
                mat[(row, col).into()] = self[(row, col).into()] + rhs[(row, col).into()]
            }
        }

        mat
    }

    fn add_assign(&mut self, rhs: &impl MatrixOps<T>) {
        assert!(self.shape_eq(rhs));
        for row in 0..self.row_size() {
            for col in 0..self.col_size() {
                self[(row, col).into()] += rhs[(row, col).into()]
            }
        }
    }

    fn sub(&self, rhs: &impl MatrixOps<T>) -> Self {
        assert!(self.shape_eq(rhs));
        let mut mat = Self::default_with_shape(self.shape());
        for row in 0..self.row_size() {
            for col in 0..self.col_size() {
                mat[(row, col).into()] = self[(row, col).into()] - rhs[(row, col).into()]
            }
        }

        mat
    }

    fn sub_assign(&mut self, rhs: &impl MatrixOps<T>) {
        assert!(self.shape_eq(rhs));
        for row in 0..self.row_size() {
            for col in 0..self.col_size() {
                self[(row, col).into()] -= rhs[(row, col).into()]
            }
        }
    }

    fn mul(&self, rhs: &impl MatrixOps<T>) -> Self {
        assert!(self.col_size() == rhs.row_size());
        let mut mat = Self::default_with_shape((self.row_size(), rhs.col_size()).into());
        for row in 0..self.row_size() {
            for col in 0..rhs.col_size() {
                for i in 0..self.col_size() {
                    mat[(row, col).into()] += self[(row, i).into()] * self[(i, col).into()];
                }
            }
        }

        mat
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Matrix<T, S> {
    shape: Index2D,
    inner: S,
    phantom: PhantomData<T>,
}

impl<T, S> Matrix<T, S> {
    pub fn new(shape: Index2D, storage: S) -> Self {
        Self {
            shape,
            inner: storage,
            phantom: PhantomData,
        }
    }
}

impl<T, S> MatrixShape for Matrix<T, S> {
    fn row_size(&self) -> usize {
        self.shape.row
    }
    fn col_size(&self) -> usize {
        self.shape.col
    }
}

impl<T> Index<Index2D> for Matrix<T, Vec<T>> {
    type Output = T;

    fn index(&self, index: Index2D) -> &Self::Output {
        &self.inner[index.to_1d(self.col_size())]
    }
}

impl<T> IndexMut<Index2D> for Matrix<T, Vec<T>> {
    fn index_mut(&mut self, index: Index2D) -> &mut Self::Output {
        let index1d = index.to_1d(self.col_size());
        &mut self.inner[index1d]
    }
}

impl<T: BaseFloat> MatrixOps<T> for Matrix<T, Vec<T>> {
    fn default_with_shape(shape: Index2D) -> Self {
        Self::new(shape, vec![T::ZERO; shape.col * shape.row])
    }
}

impl<T: BaseFloat> Add for Matrix<T, Vec<T>> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert!(self.shape_eq(&rhs));
        let mut mat = Self::default_with_shape(self.shape());
        for i in 0..(self.row_size() * self.col_size()) {
            mat.inner[i] = self.inner[i] + rhs.inner[i];
        }

        mat
    }
}

impl<T: BaseFloat> Add<&Self> for Matrix<T, Vec<T>> {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self::Output {
        assert!(self.shape_eq(rhs));
        let mut mat = Self::default_with_shape(self.shape());
        for i in 0..(self.row_size() * self.col_size()) {
            mat.inner[i] = self.inner[i] + rhs.inner[i];
        }

        mat
    }
}

impl<T: BaseFloat> AddAssign<&Self> for Matrix<T, Vec<T>> {
    fn add_assign(&mut self, rhs: &Self) {
        assert!(self.shape_eq(rhs));
        for i in 0..(self.row_size() * self.col_size()) {
            self.inner[i] = self.inner[i] + rhs.inner[i];
        }
    }
}

impl<T: BaseFloat> Sub for Matrix<T, Vec<T>> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        assert!(self.shape_eq(&rhs));
        let mut mat = Self::default_with_shape(self.shape());
        for i in 0..(self.row_size() * self.col_size()) {
            mat.inner[i] = self.inner[i] - rhs.inner[i];
        }

        mat
    }
}

impl<T: BaseFloat> Sub<&Self> for Matrix<T, Vec<T>> {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self::Output {
        assert!(self.shape_eq(rhs));
        let mut mat = Self::default_with_shape(self.shape());
        for i in 0..(self.row_size() * self.col_size()) {
            mat.inner[i] = self.inner[i] - rhs.inner[i];
        }

        mat
    }
}

impl<T: BaseFloat> SubAssign<&Self> for Matrix<T, Vec<T>> {
    fn sub_assign(&mut self, rhs: &Self) {
        assert!(self.shape_eq(rhs));
        for i in 0..(self.row_size() * self.col_size()) {
            self.inner[i] = self.inner[i] - rhs.inner[i];
        }
    }
}

impl<T: BaseFloat, Rhs: MatrixOps<T>> Mul<Rhs> for Matrix<T, Vec<T>> {
    type Output = Self;
    fn mul(self, rhs: Rhs) -> Self::Output {
        <Self as MatrixOps<T>>::mul(&self, &rhs)
    }
}
