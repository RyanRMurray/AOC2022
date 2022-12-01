use crate::utils::solver_types::{solve_simultaneous, SolutionSimultaneous};
use anyhow::Result;
use itertools::Itertools;

/// Not yet implementd
pub struct Day1Solution {}

pub fn day01(input: &str) -> Result<f32> {
    solve_simultaneous::<Day1Solution, _, _, _>(input)
}

impl SolutionSimultaneous<Vec<Vec<u32>>, u32, u32> for Day1Solution {
    fn load(input: &str) -> Result<Vec<Vec<u32>>> {
        Ok(input
            .split("\n\n")
            .map(|chunk| chunk.lines().map(|l| l.parse().unwrap()).collect())
            .collect())
    }

    fn solve(input: Vec<Vec<u32>>) -> Result<(u32, u32)> {
        let sorted = input
            .iter()
            .map(|v| v.iter().sum())
            .sorted()
            .rev()
            .collect::<Vec<_>>();

        Ok((sorted[0], sorted[0] + sorted[1] + sorted[2]))
    }
}

#[cfg(test)]
mod tests {
    use crate::{solutions::day01::Day1Solution, utils::solver_types::SolutionSimultaneous};

    #[test]
    fn test_answer() {
        // note: ugly format so we don't lead with a \n
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

        let loaded = Day1Solution::load(input).unwrap();
        let (p1, p2) = Day1Solution::solve(loaded).unwrap();
        let p1_expected = 24000;
        let p2_expected = 45000;

        assert_eq!(p1_expected, p1);
        assert_eq!(p2_expected, p2);
    }
}
