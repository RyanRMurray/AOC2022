use crate::utils::{
    load_input::load_lines,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;

//not yet implemented
pub struct Day4Solution {}

/// create a u128 with 1 bits between the a-th and b-th bits inclusive
/// we assume a b value no higher than 100
fn to_bits(a: u32, b: u32) -> u128 {
    (a..b + 1).fold(0b0, |into, i| into | 1 << i)
}

impl SolutionLinear<Vec<(u128, u128)>, usize, usize> for Day4Solution {
    /// create bitmasks for sectors covered by each elf
    fn load(input: &str) -> Result<Vec<(u128, u128)>> {
        Ok(load_lines(input, |line| {
            let nums = line.split_once(',').unwrap();
            let (a, b) = nums.0.split_once('-').unwrap();
            let (x, y) = nums.1.split_once('-').unwrap();

            (
                to_bits(a.parse().unwrap(), b.parse().unwrap()),
                to_bits(x.parse().unwrap(), y.parse().unwrap()),
            )
        }))
    }

    // mask left with right, and right with left, to see if value becomes 0 (one vec fully contains the other)
    fn part1(input: &mut Vec<(u128, u128)>) -> Result<usize> {
        Ok(input
            .iter()
            .filter(|(left, right)| (left & !right) == 0 || (right & !left) == 0)
            .count())
    }

    // mask left with right and see if the value decreases (this would only happen if there's an overlap)
    fn part2(input: &mut Vec<(u128, u128)>, _part_1_solution: usize) -> Result<usize> {
        Ok(input
            .iter()
            .filter(|(left, right)| (left & !right) < *left)
            .count())
    }
}

pub fn day04(input: &str) -> Result<f32> {
    solve_linear::<Day4Solution, _, _, _>(input)
}

#[cfg(test)]
mod tests {
    use super::Day4Solution;
    use crate::utils::solver_types::SolutionLinear;

    #[test]
    fn test_answer() {
        // note: ugly format so we don't lead with a \n
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

        let p1_expected = 2;
        let p2_expected = 4;

        let mut loaded = Day4Solution::load(input).unwrap();
        let p1 = Day4Solution::part1(&mut loaded).unwrap();
        assert_eq!(p1_expected, p1);

        let p2 = Day4Solution::part2(&mut loaded, p1).unwrap();
        assert_eq!(p2_expected, p2);
    }
}
