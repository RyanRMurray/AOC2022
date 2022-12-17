use crate::utils::solver_types::{solve_simultaneous, SolutionSimultaneous};
use anyhow::Result;

//not yet implemented
pub struct Day2Solution {}

#[derive(Eq, PartialEq)]
enum Rps {
    Rock,     //lose
    Paper,    //draw
    Scissors, //win
}

fn game((them, us): &(Rps, Rps)) -> usize {
    let a = match us {
        Rps::Rock => 1,
        Rps::Paper => 2,
        Rps::Scissors => 3,
    };

    let b = match (them, us) {
        (Rps::Rock, Rps::Paper) => 6,
        (Rps::Paper, Rps::Scissors) => 6,
        (Rps::Scissors, Rps::Rock) => 6,
        (a, b) if a == b => 3,
        _ => 0,
    };

    a + b
}

fn game2((them, strat): &(Rps, Rps)) -> usize {
    match strat {
        Rps::Rock => match them {
            Rps::Rock => 3,
            Rps::Paper => 1,
            Rps::Scissors => 2,
        },
        Rps::Paper => {
            3 + match them {
                Rps::Rock => 1,
                Rps::Paper => 2,
                Rps::Scissors => 3,
            }
        }
        Rps::Scissors => {
            6 + match them {
                Rps::Rock => 2,
                Rps::Paper => 3,
                Rps::Scissors => 1,
            }
        }
    }
}

impl SolutionSimultaneous<Vec<(Rps, Rps)>, usize, usize> for Day2Solution {
    fn load(input: &str) -> Result<Vec<(Rps, Rps)>> {
        Ok(input
            .lines()
            .map(|l| {
                (
                    match l.chars().next().unwrap() {
                        'A' => Rps::Rock,
                        'B' => Rps::Paper,
                        'C' => Rps::Scissors,
                        _ => panic!("invaliid input"),
                    },
                    match l.chars().nth(2).unwrap() {
                        'X' => Rps::Rock,
                        'Y' => Rps::Paper,
                        'Z' => Rps::Scissors,
                        _ => panic!("invaliid input"),
                    },
                )
            })
            .collect())
    }

    fn solve(input: Vec<(Rps, Rps)>) -> Result<(usize, usize)> {
        Ok(input.iter().fold((0, 0), |(p1, p2), strat| {
            (p1 + game(strat), p2 + game2(strat))
        }))
    }
}

pub fn day02(input: &str) -> Result<f32> {
    solve_simultaneous::<Day2Solution, _, _, _>(input)
}

#[cfg(test)]
mod tests {
    use super::Day2Solution;
    use crate::utils::solver_types::SolutionSimultaneous;

    #[test]
    fn test_answer() {
        // note: ugly format so we don't lead with a \n
        let input = r#"A Y
B X
C Z"#;

        let p1_expected = 15;
        let p2_expected = 12;

        let loaded = Day2Solution::load(input).unwrap();
        let (p1, p2) = Day2Solution::solve(loaded).unwrap();

        assert_eq!(p1_expected, p1);
        assert_eq!(p2_expected, p2);
    }
}
