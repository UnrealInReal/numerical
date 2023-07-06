use core::marker::PhantomData;
use core::ops::{Index, IndexMut};

pub struct Vector<T, S> {
    inner: S,
    phantom: PhantomData<T>,
}

impl<T, S> Index<usize> for Vector<T, S>
where
    S: Index<usize, Output = T>,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl<T, S> IndexMut<usize> for Vector<T, S>
where
    S: IndexMut<usize> + Index<usize, Output = T>,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

impl<T, S> Vector<T, S> {
    pub fn new(inner: S) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }
}
