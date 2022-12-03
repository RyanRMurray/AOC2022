use std::collections::HashSet;

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::Itertools;

//not yet implemented
pub struct Day3Solution {}

#[derive(Debug)]
struct Sack {
    l: HashSet<char>,
    r: HashSet<char>,
    all: HashSet<char>,
}

impl Sack {
    fn new(l: HashSet<char>, r: HashSet<char>) -> Self {
        let mut all = l.clone();
        all.extend(r.clone());
        Self { l, r, all }
    }
}

fn to_priority(c: &char) -> u32 {
    match c.is_uppercase() {
        true => *c as u32 - 38,
        _ => *c as u32 - 96,
    }
}

impl SolutionLinear<Vec<Sack>, u32, u32> for Day3Solution {
    fn load(input: &str) -> Result<Vec<Sack>> {
        Ok(input
            .lines()
            .map(|line| {
                let cs = line.chars().collect_vec();
                // populate left and right pockets
                let mut l = HashSet::new();
                let mut r = HashSet::new();
                let mid = line.len() / 2;

                for i in 0..mid {
                    l.insert(cs[i]);
                    r.insert(cs[i + mid]);
                }

                Sack::new(l, r)
            })
            .collect())
    }

    fn part1(input: &mut Vec<Sack>) -> Result<u32> {
        Ok(input
            .iter()
            .map(|s| s.l.intersection(&s.r).next().unwrap())
            .map(to_priority)
            .sum())
    }

    fn part2(input: &mut Vec<Sack>, _part_1_solution: u32) -> Result<u32> {
        Ok(input
            .chunks(3)
            .map(|ch| {
                if let [a, b, c] = ch {
                    a.all
                        .iter()
                        .find(|i| c.all.contains(i) && b.all.contains(i))
                        .unwrap()
                } else {
                    unreachable!("Invalid number of sacks. check input.")
                }
            })
            .map(to_priority)
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
