use std::collections::HashSet;

use itertools::Itertools;

#[derive(Hash, PartialEq, Eq, PartialOrd, Debug, Clone, Copy)]
pub struct Pt<const DIMS: usize>(pub [isize; DIMS]);

impl<const DIMS: usize> Default for Pt<DIMS> {
    fn default() -> Self {
        Self([0; DIMS])
    }
}

#[allow(dead_code)]
impl<const DIMS: usize> Pt<DIMS> {
    /// get all the offsets required to get every neighbour to a position
    pub fn neighbour_offsets() -> HashSet<Pt<DIMS>> {
        vec![[-1, 0, 1]; DIMS]
            .into_iter()
            .multi_cartesian_product()
            .map(|vec| vec.try_into().unwrap())
            .filter(|arr| arr != &[0; DIMS])
            .map(Pt)
            .collect()
    }

    /// get all the offsets required to get every cardinal (non-diagonal) neighbour to a position
    pub fn card_offsets() -> HashSet<Pt<DIMS>> {
        let mut pts = vec![[0; DIMS]; DIMS * 2];

        for i in 0..DIMS {
            pts[i][i] = 1;
            pts[i + DIMS][i] = -1;
        }

        pts.into_iter().map(Pt).collect()
    }

    pub fn add(&self, oth: &Pt<DIMS>) -> Self {
        let mut res = [0; DIMS];
        for (i, (a, b)) in self.0.iter().zip(oth.0).enumerate() {
            res[i] = a + b;
        }
        Pt(res)
    }

    pub fn mul(&self, multiply_by: isize) -> Self {
        let mut res = [0; DIMS];
        for (i, x) in self.0.iter().enumerate() {
            res[i] = multiply_by * x
        }
        Pt(res)
    }

    pub fn mag(&self) -> isize {
        self.0.iter().map(|v| v.abs()).sum()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::Pt;
    use rstest::rstest;

    #[test]
    fn validate_offsets() {
        let expected_2d: HashSet<Pt<2>> = vec![
            [-1, -1],
            [-1, 0],
            [-1, 1],
            [0, -1],
            [0, 1],
            [1, -1],
            [1, 0],
            [1, 1],
        ]
        .into_iter()
        .map(Pt)
        .collect();

        assert_eq!(expected_2d, Pt::<2>::neighbour_offsets());

        let expected_3d: HashSet<Pt<3>> = vec![
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
        ]
        .into_iter()
        .map(Pt)
        .collect();

        assert_eq!(expected_3d, Pt::<3>::neighbour_offsets());
    }

    #[test]
    fn validate_card_offsets() {
        let expected_2d: HashSet<Pt<2>> = vec![[-1, 0], [0, -1], [0, 1], [1, 0]]
            .into_iter()
            .map(Pt)
            .collect();

        assert_eq!(expected_2d, Pt::<2>::card_offsets());

        let expected_3d: HashSet<Pt<3>> = vec![
            [-1, 0, 0],
            [0, -1, 0],
            [0, 0, -1],
            [0, 0, 1],
            [0, 1, 0],
            [1, 0, 0],
        ]
        .into_iter()
        .map(Pt)
        .collect();

        assert_eq!(expected_3d, Pt::<3>::card_offsets());
    }

    #[rstest]
    #[case(Pt([1,2,3,4]), Pt([1,2,0,0]), Pt([0,0,3,4]))]
    #[case(Pt([-102,34,0,-3]), Pt([100,14,-10000,999]), Pt([-202,20,10000,-1002]))]
    fn validate_add(#[case] expected: Pt<4>, #[case] a: Pt<4>, #[case] b: Pt<4>) {
        assert_eq!(expected, a.add(&b))
    }

    #[rstest]
    #[case(Pt([100,200,300]), Pt([1,2,3]), 100)]
    #[case(Pt([-100,-200,-300]), Pt([1,2,3]), -100)]
    fn validate_mul(#[case] expected: Pt<3>, #[case] a: Pt<3>, #[case] b: isize) {
        assert_eq!(expected, a.mul(b))
    }
}
