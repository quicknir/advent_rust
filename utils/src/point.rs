use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Point<T, const N: usize> {
    data: [T; N],
}

impl<T: Default, const N: usize> Point<T, N> {
    pub fn new() -> Self {
        Point {
            data: std::array::from_fn(|_| Default::default()),
        }
    }
}

impl<T: Default, const N: usize> Default for Point<T, N> {
    fn default() -> Self {
        Point::<T, N>::new()
    }
}

impl<T: Clone, const N: usize> From<[T; N]> for Point<T, N> {
    fn from(value: [T; N]) -> Self {
        Point::<T, N> { data: value }
    }
}

impl<T, const N: usize> AsRef<[T; N]> for Point<T, N> {
    fn as_ref(&self) -> &[T; N] {
        &self.data
    }
}

impl<T: Add + Copy, const N: usize, U: Into<Point<T, N>>> Add<U> for Point<T, N> {
    type Output = Point<T::Output, N>;

    fn add(self, rhs: U) -> Self::Output {
        let rhs = rhs.into();
        Point {
            data: std::array::from_fn(|i| self.data[i] + rhs.data[i]),
        }
    }
}

impl<T: AddAssign + Copy, const N: usize, U: Into<Point<T, N>>> AddAssign<U> for Point<T, N> {
    fn add_assign(&mut self, rhs: U) {
        let rhs = rhs.into();
        for (i, e) in self.data.iter_mut().enumerate() {
            *e += rhs.data[i]
        }
    }
}

impl<T: Sub + Copy, const N: usize, U: Into<Point<T, N>>> Sub<U> for Point<T, N> {
    type Output = Point<T::Output, N>;

    fn sub(self, rhs: U) -> Self::Output {
        let rhs = rhs.into();
        Point {
            data: std::array::from_fn(|i| self.data[i] - rhs.data[i]),
        }
    }
}

impl<T: SubAssign + Copy, const N: usize, U: Into<Point<T, N>>> SubAssign<U> for Point<T, N> {
    fn sub_assign(&mut self, rhs: U) {
        let rhs = rhs.into();
        for (i, e) in self.data.iter_mut().enumerate() {
            *e -= rhs.data[i]
        }
    }
}

impl<T: Mul<Output = T> + Copy, const N: usize> Mul<T> for Point<T, N> {
    type Output = Point<T, N>;

    fn mul(self, rhs: T) -> Self::Output {
        let arr = std::array::from_fn(|i| rhs * self[i]);
        arr.into()
    }
}

impl<T, const RANK: usize> Index<usize> for Point<T, RANK> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const RANK: usize> IndexMut<usize> for Point<T, RANK> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
