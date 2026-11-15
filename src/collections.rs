use std::ops::*;

pub struct SwapArray<T, const N: usize> {
    items: Box<[T; N]>,
    size: usize
}

impl<T, const N: usize> SwapArray<T, N>
    where T: Copy + Clone
{
    pub fn add(&mut self, element: impl Into<T>) {
        self.items[self.size] = element.into();
        self.size += 1;
    }

    pub fn remove(&mut self, index: usize) {
        self.size -= 1;
        self.items[index] = self.items[self.size];
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == N
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl<T, const N: usize> Default for SwapArray<T, N>
    where T: Copy + Clone + Default
{
    fn default() -> Self {
        SwapArray {
            items: Box::new([T::default(); N]),
            size: 0
        }
    }
}

impl<T, const N: usize> Index<usize> for SwapArray<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for SwapArray<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}
