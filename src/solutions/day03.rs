use std::collections::HashSet;

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::Itertools;

//not yet implemented
pub struct Day3Solution {}

#[derive(Debug)]
struct Sack {
    l: HashSet<u32>,
    r: HashSet<u32>,
    all: HashSet<u32>,
}

impl Sack {
    fn new(l: HashSet<u32>, r: HashSet<u32>) -> Self {
        let mut all = l.clone();
        all.extend(r.clone());
        Self { l, r, all }
    }
}

impl SolutionLinear<Vec<Sack>, u32, u32> for Day3Solution {
    fn load(input: &str) -> Result<Vec<Sack>> {
        Ok(input
            .lines()
            .map(|line| {
                let nums: Vec<u32> = line
                    .chars()
                    .map(|c| match c.is_uppercase() {
                        true => c as u32 - 38,
                        _ => c as u32 - 96,
                    })
                    .collect_vec();

                // populate left and right pockets
                let mut l = HashSet::new();
                let mut r = HashSet::new();
                let mid = nums.len() / 2;

                for i in 0..mid {
                    l.insert(nums[i]);
                    r.insert(nums[i + mid]);
                }

                Sack::new(l, r)
            })
            .collect())
    }

    fn part1(input: &mut Vec<Sack>) -> Result<u32> {
        Ok(input
            .iter()
            .map(|s| s.l.intersection(&s.r).next().unwrap())
            .sum())
    }

    fn part2(input: &mut Vec<Sack>, _part_1_solution: u32) -> Result<u32> {
        Ok(input
            .chunks(3)
            .map(|ch| {
                if let [a, b, c] = ch {
                    let ab: HashSet<u32> = a.all.intersection(&b.all).copied().collect();

                    return *ab.intersection(&c.all).next().unwrap();
                }
                panic!("Invalid number of sacks. check input.")
            })
            .sum())
    }
}

pub fn day03(input: &str) -> Result<f32> {
    solve_linear::<Day3Solution, _, _, _>(input)
}

#[cfg(test)]
mod tests {
    use super::Day3Solution;
    use crate::utils::solver_types::SolutionLinear;

    #[test]
    fn test_answer() {
        // note: ugly format so we don't lead with a \n
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

        let p1_expected = 157;
        let p2_expected = 70;

        let mut loaded = Day3Solution::load(input).unwrap();
        let p1 = Day3Solution::part1(&mut loaded).unwrap();
        let p2 = Day3Solution::part2(&mut loaded, p1).unwrap();

        assert_eq!(p1_expected, p1);
        assert_eq!(p2_expected, p2);
    }
}
