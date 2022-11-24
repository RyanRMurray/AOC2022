use crate::utils::types::SolutionSimultaneous;
use anyhow::anyhow;
use itertools::Itertools;

// Example:
// input: [6,5,4,2,3,5,8]
// part 1: get the first number that's higher than the previous
// part 2: get the number after the first number that's higher than the previous

#[derive(Default)]
struct ExampleSolution {}

impl SolutionSimultaneous<Vec<usize>, usize, usize> for ExampleSolution {
    fn load(&self, input: &str) -> anyhow::Result<Vec<usize>> {
        Ok(input
            .replace(['[', ']'], "")
            .split(',')
            .into_iter()
            .map(|num| num.parse().unwrap())
            .collect())
    }

    fn solve(&self, input: Vec<usize>) -> anyhow::Result<(usize, usize)> {
        let mut prev: usize = usize::MAX;
        for (x, y) in input.iter().tuple_windows() {
            if x > &prev {
                return Ok((*x, *y));
            }
            prev = *x;
        }
        Err(anyhow!("Invalid input - check and try again"))
    }
}

#[cfg(test)]
mod tests {
    use super::ExampleSolution;
    use crate::utils::types::SolutionSimultaneous;
    use rstest::rstest;

    #[rstest]
    #[case("[6,5,4,2,3,5,8]", 3, 5)]
    #[case("[1,2,6,4,100]", 2, 6)]
    #[case("[5,4,3,1,2,7]", 2, 7)]
    fn validate_simul(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let solver = ExampleSolution::default();

        let input = solver.load(input).unwrap();
        let (p1, p2) = solver.solve(input).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
