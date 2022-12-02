use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;

//not yet implemented
pub struct Day2Solution {}

#[derive(Eq, PartialEq)]
enum RPS {
    Rock,     //lose
    Paper,    //draw
    Scissors, //win
}

fn game((them, us): &(RPS, RPS)) -> usize {
    let a = match us {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    };

    let b = match (them, us) {
        (RPS::Rock, RPS::Paper) => 6,
        (RPS::Paper, RPS::Scissors) => 6,
        (RPS::Scissors, RPS::Rock) => 6,
        (a, b) if a == b => 3,
        _ => 0,
    };

    a + b
}

fn game2((them, strat): &(RPS, RPS)) -> usize {
    match strat {
        RPS::Rock => match them {
            RPS::Rock => 3,
            RPS::Paper => 1,
            RPS::Scissors => 2,
        },
        RPS::Paper => {
            3 + match them {
                RPS::Rock => 1,
                RPS::Paper => 2,
                RPS::Scissors => 3,
            }
        }
        RPS::Scissors => {
            6 + match them {
                RPS::Rock => 2,
                RPS::Paper => 3,
                RPS::Scissors => 1,
            }
        }
    }
}

impl SolutionLinear<Vec<(RPS, RPS)>, usize, usize> for Day2Solution {
    fn load(input: &str) -> Result<Vec<(RPS, RPS)>> {
        Ok(input
            .lines()
            .map(|l| {
                (
                    match l.chars().next().unwrap() {
                        'A' => RPS::Rock,
                        'B' => RPS::Paper,
                        'C' => RPS::Scissors,
                        _ => panic!("invaliid input"),
                    },
                    match l.chars().nth(2).unwrap() {
                        'X' => RPS::Rock,
                        'Y' => RPS::Paper,
                        'Z' => RPS::Scissors,
                        _ => panic!("invaliid input"),
                    },
                )
            })
            .collect())
    }

    fn part1(input: &mut Vec<(RPS, RPS)>) -> Result<usize> {
        Ok(input.iter().map(game).sum())
    }

    fn part2(input: &mut Vec<(RPS, RPS)>, _part_1_solution: usize) -> Result<usize> {
        Ok(input.iter().map(game2).sum())
    }
}

pub fn day02(input: &str) -> Result<f32> {
    solve_linear::<Day2Solution, _, _, _>(input)
}

#[cfg(test)]
mod tests {
    use super::Day2Solution;
    use crate::utils::solver_types::SolutionLinear;

    #[test]
    fn test_answer() {
        // note: ugly format so we don't lead with a \n
        let input = r#"A Y
B X
C Z"#;

        let p1_expected = 15;
        let p2_expected = 12;

        let mut loaded = Day2Solution::load(input).unwrap();
        let p1 = Day2Solution::part1(&mut loaded).unwrap();
        let p2 = Day2Solution::part2(&mut loaded, p1).unwrap();

        assert_eq!(p1_expected, p1);
        assert_eq!(p2_expected, p2);
    }
}
