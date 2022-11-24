use std::collections::HashMap;

use super::point::Pt;

#[allow(dead_code)]
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
