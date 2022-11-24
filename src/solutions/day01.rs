use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};

/// Not yet implementd
#[derive(Default)]
pub struct Day1Solution {}

pub fn day01(input: &str) -> Result<f32> {
    solve_linear(input, &Day1Solution::default())
}

impl SolutionLinear<String, String, String> for Day1Solution {
    fn load(&self, _input: &str) -> Result<String> {
        Err(anyhow!("load not yet implemented"))
    }

    fn part1(&self, _input: &String) -> Result<String> {
        Err(anyhow!("part 1 not yet implemented"))
    }

    fn part2(&self, _input: &String, _part_1_solution: String) -> Result<String> {
        Err(anyhow!("part 2 not yet implemented"))
    }
}

#[cfg(test)]
mod tests {}
