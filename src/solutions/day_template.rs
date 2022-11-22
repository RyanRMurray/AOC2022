use crate::utils::types::SolutionLinear;

// Example:
// input: [1,2,3,4,5]
// part 1: sum up these numbers
// part 2: multiply the result of part 1 by the number of numbers in the input
#[derive(Default)]
struct ExampleSolution {
    part_1_input: Option<Vec<usize>>,
    part_2_input: Option<Vec<usize>>,
    part_1_solution: Option<usize>,
    part_2_solution: Option<usize>,
}

impl SolutionLinear<Vec<usize>, Vec<usize>, usize, usize> for ExampleSolution {
    fn get_part_1_input(&self) -> &Vec<usize> {
        match &self.part_1_input {
            None => panic!("part 1 input missing"),
            Some(val) => val,
        }
    }

    fn get_part_1_solution(&self) -> &usize {
        match &self.part_1_solution {
            None => panic!("part 1 solution missing"),
            Some(val) => val,
        }
    }

    fn get_part_2_input(&self) -> &Vec<usize> {
        match &self.part_2_input {
            None => panic!("part 2 input missing"),
            Some(val) => val,
        }
    }

    fn get_part_2_solution(&self) -> &usize {
        match &self.part_2_solution {
            None => panic!("part 2 solution missing"),
            Some(val) => val,
        }
    }

    fn load(&mut self, input: &str) {
        self.part_1_input = Some(
            input
                .replace(['[', ']'], "")
                .split(',')
                .into_iter()
                .map(|num| num.parse().unwrap())
                .collect(),
        )
    }

    fn part1(&mut self) {
        self.part_1_solution = Some(self.get_part_1_input().iter().sum())
    }

    fn part2(&mut self) {
        self.part_2_solution = Some(self.get_part_1_input().len() * self.get_part_1_solution())
    }
}

#[cfg(test)]
mod tests {
    use super::ExampleSolution;
    use crate::utils::types::{solve_linear, SolutionLinear};
    use rstest::rstest;

    #[rstest]
    #[case("[1,2,3]", 6, 18)]
    #[case("[0,7,13,20,1,100]", 141, 846)]
    #[case("[6000]", 6000, 6000)]
    fn validate_linear(#[case] input: &str, #[case] expected_1: usize, #[case] expected_2: usize) {
        let mut solver = ExampleSolution::default();

        solve_linear(input, &mut solver);

        assert_eq!(expected_1, *solver.get_part_1_solution());
        assert_eq!(expected_2, *solver.get_part_2_solution());
    }
}
