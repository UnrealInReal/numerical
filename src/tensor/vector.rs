use core::marker::PhantomData;
use core::ops::{Index, IndexMut};

pub trait Len {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

macro_rules! impl_vec_storage {
    ( $( $n:literal ),* ) => {
        $(
            impl<T> Len for [T; $n] {
                #[inline]
                fn len(&self) -> usize {
                    $n
                }
                #[inline]
                fn is_empty(&self) -> bool {
                    $n == 0
                }
            }

            impl<T> VecStorage<T> for [T; $n] {}
        )*
    };
}

impl_vec_storage!(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32
);

impl<T> Len for Vec<T> {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> VecStorage<T> for Vec<T> {}

pub trait VecStorage<T>: Len + IndexMut<usize> + Index<usize, Output = T> {}

pub struct Vector<T, S: VecStorage<T>> {
    inner: S,
    phantom: PhantomData<T>,
}

impl<T, S: VecStorage<T>> Vector<T, S> {
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }
}

impl<T, S: VecStorage<T>> Len for Vector<T, S> {
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T, S: VecStorage<T>> Index<usize> for Vector<T, S> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<T, S: VecStorage<T>> IndexMut<usize> for Vector<T, S>
where
    S: IndexMut<usize> + Index<usize, Output = T>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}
