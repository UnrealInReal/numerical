use core::marker::PhantomData;
use core::ops::{Index, IndexMut};

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

pub trait Shape {
    fn row_size(&self) -> usize;
    fn col_size(&self) -> usize;
    fn shape(&self) -> Index2D {
        Index2D {
            row: self.row_size(),
            col: self.col_size(),
        }
    }
}

pub trait MatrixOps<T>: Shape + Index<Index2D, Output = T> + IndexMut<Index2D> {}

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

impl<T, S> Shape for Matrix<T, S> {
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

impl<T> MatrixOps<T> for Matrix<T, Vec<T>> {}
