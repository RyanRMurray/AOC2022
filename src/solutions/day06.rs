use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::Itertools;

//not yet implemented
pub struct Day6Solution {}

fn first_with_n_unique(input: Vec<char>, n: usize) -> usize {
    for (i, tup) in (0..).zip(input.windows(n)) {
        if tup.iter().all_unique() {
            return i + n;
        }
    }
    unreachable!("Check input and try again")
}

impl SolutionLinear<Vec<char>, usize, usize> for Day6Solution {
    fn load(input: &str) -> Result<Vec<char>> {
        Ok(input.chars().collect())
    }

    fn part1(input: &mut Vec<char>) -> Result<usize> {
        Ok(first_with_n_unique(input.to_vec(), 4))
    }

    fn part2(input: &mut Vec<char>, _part_1_solution: usize) -> Result<usize> {
        Ok(first_with_n_unique(input.to_vec(), 14))
    }
}

pub fn day06(input: &str) -> Result<f32> {
    solve_linear::<Day6Solution, _, _, _>(input)
}

#[cfg(test)]
mod tests {
    use super::Day6Solution;
    use crate::utils::solver_types::SolutionLinear;

    #[rstest::rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6, 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26)]
    fn test_answer(#[case] input: &str, #[case] p1_expected: usize, #[case] p2_expected: usize) {
        let mut loaded = Day6Solution::load(input).unwrap();

        let p1 = Day6Solution::part1(&mut loaded).unwrap();
        assert_eq!(p1_expected, p1);

        let p2 = Day6Solution::part2(&mut loaded, p1).unwrap();
        assert_eq!(p2_expected, p2);
    }
}
