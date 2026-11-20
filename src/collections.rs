use std::ops::*;
use std::iter;

pub struct SwapArray<T, const N: usize> {
    items: Box<[T; N]>,
    size: usize
}

impl<T, const N: usize> SwapArray<T, N>
    where T: Copy + Clone
{
    pub fn add(&mut self, item: impl Into<T>) {
        self.items[self.size] = item.into();
        self.size += 1;
    }

    pub fn remove(&mut self, index: usize) {
        self.size -= 1;
        self.items[index] = self.items[self.size];
    }

    pub fn retain<F>(&mut self, f: F)
        where F: Fn(&T) -> bool
    {
        let mut i = 0;
        while i < self.size {
            if !f(&self.items[i]) {
                self.remove(i);
            }
            i += 1;
        }
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

impl<T, const N: usize> IntoIterator for SwapArray<T, N> {
    type Item = T;
    type IntoIter = iter::Take<std::array::IntoIter<T, N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter().take(N)
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a SwapArray<T, N> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items[0..self.size].iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut SwapArray<T, N> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items[0..self.size].iter_mut()
    }
}

pub struct CircleArray<T, const N: usize> {
    items: Box<[T; N]>,
    head: usize
}

impl<T, const N: usize> CircleArray<T, N>
    where T: Copy + Clone
{
    pub fn new(array: impl Into<[T; N]>) -> CircleArray<T, N> {
        CircleArray {
            items: Box::new(array.into()),
            head: 0
        }
    }

    pub fn add(&mut self, item: impl Into<T>) {
        self.items[self.head] = item.into();
        self.head = (self.head + 1) % N;
    }

    pub fn head(&self) -> usize {
        self.head
    }
}

impl<T, const N: usize> Default for CircleArray<T, N>
    where T: Copy + Clone + Default
{
    fn default() -> Self {
        CircleArray {
            items: Box::new([T::default(); N]),
            head: 0
        }
    }
}

impl<T, const N: usize> Index<usize> for CircleArray<T, N> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[(self.head + index) % N]
    }
}

impl<T, const N: usize> IndexMut<usize> for CircleArray<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[(self.head + index) % N]
    }
}

impl<T, const N: usize> IntoIterator for CircleArray<T, N>
    where T: Clone {
    type Item = T;
    type IntoIter = std::iter::Chain<std::vec::IntoIter<T>, std::vec::IntoIter<T>>;

    fn into_iter(self) -> Self::IntoIter {
        let split = self.items.split_at(self.head);
        let first = split.0.to_vec();
        let second = split.1.to_vec();
        first.into_iter().chain(second.into_iter())
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a CircleArray<T, N> {
    type Item = &'a T;
    type IntoIter = iter::Take<iter::Skip<iter::Cycle<std::slice::Iter<'a, T>>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter().cycle().skip(self.head).take(N)
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut CircleArray<T, N> {
    type Item = &'a mut T;
    type IntoIter = iter::Chain<std::slice::IterMut<'a, T>, std::slice::IterMut<'a, T>>;

    fn into_iter(self) -> Self::IntoIter {
        let split = self.items.split_at_mut(self.head);
        split.0.iter_mut().chain(split.1.iter_mut())
    }
}
