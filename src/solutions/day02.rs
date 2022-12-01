use crate::utils::solver_types::SolutionLinear;
use anyhow::{anyhow, Result};

//not yet implemented
pub struct Day2Solution {}

impl SolutionLinear<Vec<usize>, usize, usize> for Day2Solution {
    fn load(_input: &str) -> Result<Vec<usize>> {
        Err(anyhow!("Not yet implemented"))
    }

    fn part1(_input: &mut Vec<usize>) -> Result<usize> {
        Err(anyhow!("Not yet implemented"))
    }

    fn part2(_input: &mut Vec<usize>, _part_1_solution: usize) -> Result<usize> {
        Err(anyhow!("Not yet implemented"))
    }
}

#[cfg(test)]
mod tests {
    use super::Day2Solution;
    use crate::utils::solver_types::SolutionLinear;

    #[ignore]
    #[test]
    fn test_answer() {
        // note: ugly format so we don't lead with a \n
        let input = r#"some string"#;

        let p1_expected = 0;
        let p2_expected = 0;

        let mut loaded = Day2Solution::load(input).unwrap();
        let p1 = Day2Solution::part1(&mut loaded).unwrap();
        let p2 = Day2Solution::part2(&mut loaded, p1).unwrap();

        assert_eq!(p1_expected, p1);
        assert_eq!(p2_expected, p2);
    }
}
