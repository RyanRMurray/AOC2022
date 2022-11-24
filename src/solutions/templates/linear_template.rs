use crate::utils::solver_types::SolutionLinear;
use anyhow::Result;

// Example:
// input: [1,2,3,4,5]
// part 1: sum up these numbers
// part 2: multiply the result of part 1 by the number of numbers in the input
#[derive(Default)]
pub struct ExampleSolutionLinear {}

impl SolutionLinear<Vec<usize>, usize, usize> for ExampleSolutionLinear {
    fn load(&self, input: &str) -> Result<Vec<usize>> {
        Ok(input
            .replace(['[', ']'], "")
            .split(',')
            .into_iter()
            .map(|num| num.parse().unwrap())
            .collect())
    }

    fn part1(&self, input: &Vec<usize>) -> Result<usize> {
        Ok(input.iter().sum())
    }

    fn part2(&self, input: &Vec<usize>, part_1_solution: usize) -> Result<usize> {
        Ok(input.len() * part_1_solution)
    }
}

#[cfg(test)]
mod tests {
    use super::ExampleSolutionLinear;
    use crate::utils::solver_types::SolutionLinear;
    use rstest::rstest;

    #[rstest]
    #[case("[1,2,3]", 6, 18)]
    #[case("[0,7,13,20,1,100]", 141, 846)]
    #[case("[6000]", 6000, 6000)]
    fn validate_linear(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let solver = ExampleSolutionLinear::default();

        let input = solver.load(input).unwrap();
        let p1 = solver.part1(&input).unwrap();
        let p2 = solver.part2(&input, p1).unwrap();

        assert_eq!(expected_1, p1);
        assert_eq!(expected_2, p2);
    }
}
