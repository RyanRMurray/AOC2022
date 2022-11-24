use itertools::Itertools;

#[derive(Hash, PartialEq, Eq, PartialOrd, Debug)]
pub struct Pt<const DIMS: usize>([isize; DIMS]);

impl<const DIMS: usize> Default for Pt<DIMS> {
    fn default() -> Self {
        Self([0; DIMS])
    }
}

#[allow(dead_code)]
impl<const DIMS: usize> Pt<DIMS> {
    /// get all the offsets required to get every neighbour to a position
    pub fn neighbour_offsets() -> Vec<Pt<DIMS>> {
        vec![[-1, 0, 1]; DIMS]
            .into_iter()
            .multi_cartesian_product()
            .map(|vec| vec.try_into().unwrap())
            .filter(|arr| arr != &[0; DIMS])
            .map(Pt)
            .collect_vec()
    }

    pub fn add(&self, oth: Pt<DIMS>) -> Self {
        let mut res = [0; DIMS];
        for (i, (a, b)) in self.0.iter().zip(oth.0).enumerate() {
            res[i] = a + b;
        }
        Pt(res)
    }
}

#[cfg(test)]
mod tests {
    use super::Pt;
    use rstest::rstest;

    #[test]
    fn validate_offsets() {
        let expected_2d: Vec<Pt<2>> = vec![
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

        let expected_3d: Vec<Pt<3>> = vec![
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

    #[rstest]
    #[case(Pt([1,2,3,4]), Pt([1,2,0,0]), Pt([0,0,3,4]))]
    #[case(Pt([-102,34,0,-3]), Pt([100,14,-10000,999]), Pt([-202,20,10000,-1002]))]
    fn validate_add(#[case] expected: Pt<4>, #[case] a: Pt<4>, #[case] b: Pt<4>) {
        assert_eq!(expected, a.add(b))
    }
}
