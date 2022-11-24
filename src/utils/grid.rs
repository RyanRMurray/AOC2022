use itertools::Itertools;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Default)]
struct Grid<T, const DIMS: usize> {
    pub grid: HashMap<[isize; DIMS], T>,
}

#[allow(dead_code)]
impl<T, const DIMS: usize> Grid<T, DIMS> {
    /// get all the offsets required to get every neighbour to a position in the grid
    fn neighbour_offsets(&self) -> Vec<[isize; DIMS]> {
        vec![[-1, 0, 1]; DIMS]
            .into_iter()
            .multi_cartesian_product()
            .map(|vec| vec.try_into().unwrap())
            .filter(|arr| arr != &[0; DIMS])
            .collect_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::Grid;

    #[test]
    fn validate_offsets() {
        let g2: Grid<usize, 2> = Grid::default();

        let expected = vec![
            [-1, -1],
            [-1, 0],
            [-1, 1],
            [0, -1],
            [0, 1],
            [1, -1],
            [1, 0],
            [1, 1],
        ];

        assert_eq!(expected, g2.neighbour_offsets());
        let g3: Grid<usize, 3> = Grid::default();

        let expected = vec![
            [-1, -1, -1],
            [-1, -1, 0],
            [-1, -1, 1],
            [-1, 0, -1],
            [-1, 0, 0],
            [-1, 0, 1],
            [-1, 1, -1],
            [-1, 1, 0],
            [-1, 1, 1],
            [0, -1, -1],
            [0, -1, 0],
            [0, -1, 1],
            [0, 0, -1],
            [0, 0, 1],
            [0, 1, -1],
            [0, 1, 0],
            [0, 1, 1],
            [1, -1, -1],
            [1, -1, 0],
            [1, -1, 1],
            [1, 0, -1],
            [1, 0, 0],
            [1, 0, 1],
            [1, 1, -1],
            [1, 1, 0],
            [1, 1, 1],
        ];

        assert_eq!(expected, g3.neighbour_offsets());
    }
}
