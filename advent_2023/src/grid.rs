use crate::Point;

pub type Coord<const RANK: usize> = Point<i64, RANK>;

pub struct Grid<T, const RANK: usize> {
    data: Vec<T>,
    dims: [usize; RANK],
    multipliers: [usize; RANK],
}

impl<T: Clone, const RANK: usize> Grid<T, RANK> {
    pub fn new(t: T, dims: &[usize; RANK]) -> Grid<T, RANK> {
        let mut cum_product = 1;
        let multipliers = std::array::from_fn(|i| {
            let x = cum_product;
            cum_product *= dims[i];
            x
        });
        Grid {
            data: vec![t; RANK],
            dims: *dims,
            multipliers,
        }
    }

    pub fn contains<U: Into<Coord<RANK>>>(&self, coord: U) -> bool {
        self.dims
            .iter()
            .zip(coord.into().as_ref())
            .all(|(x, y)| (*y as usize) < *x)
    }

    pub fn get<U: Into<Coord<RANK>>>(&self, coord: U) -> Option<&T> {
        self.convert_index(coord).map(|i| &self.data[i])
    }
    pub fn get_mut<U: Into<Coord<RANK>>>(&mut self, coord: U) -> Option<&mut T> {
        self.convert_index(coord).map(|i| &mut self.data[i])
    }

    fn convert_index<U: Into<Coord<RANK>>>(&self, coord: U) -> Option<usize> {
        let coord = coord.into();
        if !self.contains(coord) {
            return None;
        }
        let dot_product = self.multipliers.into_iter().zip(coord.as_ref()).fold(0, |acc, (x, y)| acc + x * (*y as usize));
        return Some(dot_product);
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
    }
    #[test]
    fn index_map() {
        let grid = Grid::new(0i32, &[3, 4, 5]);
        let mut x = HashSet::new();
        for i in 0..3 {
            for j in 0..4 {
                for k in 0..5 {
                    assert!(x.insert(grid.convert_index([i,j,k]).unwrap()));
                }
            }
        }
        assert_eq!(x.len(), 3*4*5);
        for e in &x {
            assert!(*e < 3*4*5);
        }
    }
}
