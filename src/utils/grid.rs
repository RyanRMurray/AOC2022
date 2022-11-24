use std::collections::HashMap;

use super::point::Pt;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T, const DIMS: usize> {
    offsets: Vec<Pt<DIMS>>,
    pub grid: HashMap<Pt<DIMS>, T>,
}

impl<T, const DIMS: usize> Default for Grid<T, DIMS> {
    fn default() -> Self {
        Self {
            offsets: Pt::<DIMS>::neighbour_offsets(),
            grid: Default::default(),
        }
    }
}

impl<T, const DIMS: usize> From<Vec<(Pt<DIMS>, T)>> for Grid<T, DIMS> {
    fn from(v: Vec<(Pt<DIMS>, T)>) -> Self {
        Self {
            offsets: Pt::<DIMS>::neighbour_offsets(),
            grid: v.into_iter().collect(),
        }
    }
}

#[allow(dead_code)]
impl<T, const DIMS: usize> Grid<T, DIMS> {
    /// merge one grid into this one, overwriting any existing values
    fn merge(&mut self, other: Grid<T, DIMS>) {
        other.grid.into_iter().for_each(|(k, v)| {
            self.grid.insert(k, v);
        });
    }

    /// offset the grid in the specified direction
    fn offset_all(mut self, offset_by: &Pt<DIMS>) -> Self {
        let mut new_grid = HashMap::default();
        self.grid.into_iter().for_each(|(k, v)| {
            new_grid.insert(k.add(offset_by), v);
        });
        self.grid = new_grid;

        self
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;
    use crate::utils::point::Pt;

    #[test]
    fn test_offset() {
        let expected = Grid::<i32, 2>::from(vec![
            (Pt([50, 50]), 10),
            (Pt([25, 50]), 204),
            (Pt([0, 0]), 66),
        ]);

        let input = Grid::<i32, 2>::from(vec![
            (Pt([25, 25]), 10),
            (Pt([0, 25]), 204),
            (Pt([-25, -25]), 66),
        ]);

        let result = input.offset_all(&Pt([25, 25]));

        assert_eq!(expected, result);
    }
}