use std::collections::VecDeque;

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::Itertools;

//not yet implemented
pub struct Day6Solution {}

impl SolutionLinear<VecDeque<char>, usize, usize> for Day6Solution {
    fn load(input: &str) -> Result<VecDeque<char>> {
        Ok(input.chars().collect())
    }

    fn part1(input: &mut VecDeque<char>) -> Result<usize> {
        let mut buffer = input.clone();
        let rest = buffer.split_off(3);

        for (i, c) in rest.iter().enumerate() {
            buffer.push_back(*c);
            if buffer.iter().unique().count() == 4 {
                return Ok(i + 4); // add four to account for offset
            }
            buffer.pop_front();
        }
        unreachable!("Check input and try again")
    }

    fn part2(input: &mut VecDeque<char>, _part_1_solution: usize) -> Result<usize> {
        let rest = input.split_off(13);

        for (i, c) in rest.iter().enumerate() {
            input.push_back(*c);
            if input.iter().unique().count() == 14 {
                return Ok(i + 14); // add fourteen to account for offset
            }
            input.pop_front();
        }
        unreachable!("Check input and try again")
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
