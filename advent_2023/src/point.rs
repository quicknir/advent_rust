use std::ops::{Add, AddAssign, Sub, SubAssign};


#[derive(Debug, Clone, Copy)]
pub struct Point<T, const N: usize> {
    data: [T; N],
}

impl<T: Default, const N: usize> Point<T, N> {
    fn new() -> Self {
        Point{data: std::array::from_fn(|_| Default::default())}
    }

}

impl<T: Default, const N: usize> Default for Point<T, N> {
    fn default() -> Self {
        Point::<T, N>::new()
    }
}

impl <T: Clone, const N: usize> From<[T; N]> for Point<T, N> {
    fn from(value: [T; N]) -> Self {
        Point::<T, N>{data: value}
    }
}

impl<T, const N: usize> AsRef<[T; N]> for Point<T, N> {
    fn as_ref(&self) -> &[T; N] {
        &self.data
    }
}

impl<T: Add+Copy, const N:usize, U: Into<Point<T, N>>> Add<U> for Point<T, N> {
    type Output = Point<T::Output, N>;

    fn add(self, rhs: U) -> Self::Output {
        let rhs = rhs.into();
        Point{ data: std::array::from_fn(|i| self.data[i] + rhs.data[i])}
    }
}

impl<T: AddAssign+Copy, const N:usize, U: Into<Point<T, N>>> AddAssign<U> for Point<T, N> {
    fn add_assign(&mut self, rhs: U) {
        let rhs = rhs.into();
        for (i, e) in self.data.iter_mut().enumerate() {
            *e += rhs.data[i]
        }
    }
}

impl<T: Sub+Copy, const N:usize, U: Into<Point<T, N>>> Sub<U> for Point<T, N> {
    type Output = Point<T::Output, N>;

    fn sub(self, rhs: U) -> Self::Output {
        let rhs = rhs.into();
        Point{ data: std::array::from_fn(|i| self.data[i] - rhs.data[i])}
    }
}

impl<T: SubAssign+Copy, const N:usize, U: Into<Point<T, N>>> SubAssign<U> for Point<T, N> {
    fn sub_assign(&mut self, rhs: U) {
        let rhs = rhs.into();
        for (i, e) in self.data.iter_mut().enumerate() {
            *e -= rhs.data[i]
        }
    }
}
