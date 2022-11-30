use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::{anyhow, Result};

/// Not yet implementd
pub struct Day1Solution {}

pub fn day01(input: &str) -> Result<f32> {
    solve_linear::<Day1Solution, _, _, _>(input)
}

impl SolutionLinear<String, String, String> for Day1Solution {
    fn load(_input: &str) -> Result<String> {
        Err(anyhow!("load not yet implemented"))
    }

    fn part1(_input: &mut String) -> Result<String> {
        Err(anyhow!("part 1 not yet implemented"))
    }

    fn part2(_input: &mut String, _part_1_solution: String) -> Result<String> {
        Err(anyhow!("part 2 not yet implemented"))
    }
}

#[cfg(test)]
mod tests {}
