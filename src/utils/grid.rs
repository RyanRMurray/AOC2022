use std::collections::{HashMap, HashSet};

use super::point::Pt;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T, const DIMS: usize> {
    /// neighbour offsets for points in this N dimensions
    offsets: HashSet<Pt<DIMS>>,
    /// cardinal offsets for points in this N dimensions
    card_offsets: HashSet<Pt<DIMS>>,
    default_val: T,
    pub grid: HashMap<Pt<DIMS>, T>,
}

impl<T: Default, const DIMS: usize> Default for Grid<T, DIMS> {
    fn default() -> Self {
        Self {
            offsets: Pt::<DIMS>::neighbour_offsets(),
            card_offsets: Pt::<DIMS>::card_offsets(),
            default_val: T::default(),
            grid: Default::default(),
        }
    }
}

impl<T: Default, const DIMS: usize> From<Vec<(Pt<DIMS>, T)>> for Grid<T, DIMS> {
    fn from(v: Vec<(Pt<DIMS>, T)>) -> Self {
        Self {
            offsets: Pt::<DIMS>::neighbour_offsets(),
            card_offsets: Pt::<DIMS>::card_offsets(),
            default_val: T::default(),
            grid: v.into_iter().collect(),
        }
    }
}

#[allow(dead_code)]
impl<T, const DIMS: usize> Grid<T, DIMS> {
    /// merge one grid into this one, using the specified merge_function
    fn merge(&mut self, other: Grid<T, DIMS>, merge_function: fn(&T, &T) -> T) {
        other.grid.into_iter().for_each(|(k, v)| {
            let new_val = merge_function(self.grid.get(&k).unwrap_or(&self.default_val), &v);
            self.grid.insert(k, new_val);
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

    #[test]
    fn test_merge() {
        let mut target = Grid::<i32, 2>::from(vec![(Pt([50, 50]), 10), (Pt([25, 50]), 204)]);
        let to_merge = Grid::<i32, 2>::from(vec![(Pt([25, 50]), 5000), (Pt([0, 0]), 60)]);

        let expected = Grid::<i32, 2>::from(vec![
            (Pt([50, 50]), 10),
            (Pt([25, 50]), 5000),
            (Pt([0, 0]), 60),
        ]);

        target.merge(to_merge, |_, x| *x);

        assert_eq!(expected, target);
    }
}
