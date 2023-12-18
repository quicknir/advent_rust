use std::ops::{Index, IndexMut};

use crate::Point;

pub type Coord<const RANK: usize> = Point<i64, RANK>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid<T, const RANK: usize> {
    data: Vec<T>,
    dims: [i64; RANK],
    multipliers: [i64; RANK],
}

impl<T: Clone, const RANK: usize> Grid<T, RANK> {
    pub fn new(t: T, dims: &[i64; RANK]) -> Grid<T, RANK> {
        let mut cum_product = 1;
        let multipliers = std::array::from_fn(|i| {
            let x = cum_product;
            cum_product *= dims[i];
            x
        });
        Grid {
            data: vec![t; cum_product as usize],
            dims: *dims,
            multipliers,
        }
    }
    pub fn from_data(data: Vec<T>, dims: &[i64; RANK]) -> Grid<T, RANK> {
        let mut cum_product = 1;
        let multipliers = std::array::from_fn(|i| {
            let x = cum_product;
            cum_product *= dims[i];
            x
        });
        assert_eq!(data.len(), cum_product as usize);
        Grid {
            data,
            dims: *dims,
            multipliers,
        }
    }
    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }
}

impl<T, const RANK: usize> Grid<T, RANK> {
    pub fn contains<U: Into<Coord<RANK>>>(&self, coord: U) -> bool {
        self.dims
            .iter()
            .zip(coord.into().as_ref())
            .all(|(x, y)| (*y as u64) < (*x as u64))
    }

    pub fn get<U: Into<Coord<RANK>>>(&self, coord: U) -> Option<&T> {
        self.convert_index(coord).map(|i| &self.data[i])
    }
    pub fn get_mut<U: Into<Coord<RANK>>>(&mut self, coord: U) -> Option<&mut T> {
        self.convert_index(coord).map(|i| &mut self.data[i])
    }

    pub fn get_dims(&self) -> [i64; RANK] {
        self.dims
    }

    fn convert_index<U: Into<Coord<RANK>>>(&self, coord: U) -> Option<usize> {
        let coord = coord.into();
        if !self.contains(coord) {
            return None;
        }
        let dot_product = self
            .multipliers
            .into_iter()
            .zip(coord.as_ref())
            .fold(0, |acc: i64, (x, y)| acc + x * *y);
        return Some(dot_product as usize);
    }
}

impl Grid<bool, 2> {
    pub fn print(&self) {
        for row in 0..self.dims[1] {
            let s: String = (0..self.dims[0])
                .map(|col| if self[[col, row]] { '*' } else { '.' })
                .collect();
            println!("{}", s);
        }
    }
}

impl<T, const RANK: usize, Idx: Into<Coord<RANK>>> Index<Idx> for Grid<T, RANK> {
    type Output = T;

    fn index(&self, index: Idx) -> &Self::Output {
        let index = index.into();
        self.get(index)
            .unwrap_or_else(|| panic!("Invalid index {:?}", index))
    }
}

impl<T, const RANK: usize, Idx: Into<Coord<RANK>>> IndexMut<Idx> for Grid<T, RANK> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        let index = index.into();
        self.get_mut(index)
            .unwrap_or_else(|| panic!("Invalid index {:?}", index))
    }
}

#[cfg(test)]
mod tests {

    use crate::*;
    #[test]
    fn test_contains() {
        let grid = Grid::new(0i32, &[3, 3, 3]);
        assert!(grid.contains([1, 1, 1]));
        assert!(!grid.contains([3, 1, 1]));
        let coord: Coord<3> = [3, 1, 1].into();
        assert!(!grid.contains(coord));
        assert!(!grid.contains([-1, 0, 0]));
    }
    #[test]
    fn index_map() {
        let grid = Grid::new(0i32, &[3, 4, 5]);
        let mut x = HashSet::new();
        for i in 0..3 {
            for j in 0..4 {
                for k in 0..5 {
                    assert!(x.insert(grid.convert_index([i, j, k]).unwrap()));
                }
            }
        }
        assert_eq!(x.len(), 3 * 4 * 5);
        for e in &x {
            assert!(*e < 3 * 4 * 5);
        }
    }
}
