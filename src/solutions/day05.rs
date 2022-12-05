use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::Itertools;

//not yet implemented
pub struct Day5Solution {}

type StacksAndArgs = (Vec<Vec<char>>, Vec<(usize, usize, usize)>); // move A from B to C

impl SolutionLinear<StacksAndArgs, String, String> for Day5Solution {
    fn load(input: &str) -> Result<StacksAndArgs> {
        let (l_stacks, l_instrs) = input.split_once("\n\n").unwrap();

        // get stacks
        let stack_contents = l_stacks
            .lines()
            .map(|l| {
                l.chars()
                    .chunks(4)
                    .into_iter()
                    .map(|cs| match cs.into_iter().nth(1).unwrap() {
                        ' ' => None,
                        c => Some(c),
                    })
                    .collect::<Vec<Option<char>>>()
            })
            .collect_vec();

        let n_stacks = stack_contents[0].len();
        let mut stacks = vec![vec![]; n_stacks];

        for i in (0..n_stacks).rev() {
            for (j, st) in stacks.iter_mut().enumerate().take(n_stacks) {
                match stack_contents[i][j] {
                    None => (),
                    Some(c) => st.push(c),
                }
            }
        }

        // get instrs
        let instrs = l_instrs
            .lines()
            .map(|l| {
                let mut words = l.split(' ');

                (
                    words.nth(1).unwrap().parse::<usize>().unwrap(),
                    words.nth(1).unwrap().parse::<usize>().unwrap() - 1,
                    words.nth(1).unwrap().parse::<usize>().unwrap() - 1,
                )
            })
            .collect_vec();

        Ok((stacks, instrs))
    }

    fn part1(input: &mut StacksAndArgs) -> Result<String> {
        let mut stacks = input.0.clone();

        // apply instrs
        for (n, from, to) in &input.1 {
            for _ in 0..*n {
                let cr = stacks[*from].pop().unwrap();
                stacks[*to].push(cr);
            }
        }

        // get tops
        let mut res = String::from("");
        for stack in stacks {
            res.push(*stack.last().unwrap());
        }

        Ok(res.to_string())
    }

    fn part2(input: &mut StacksAndArgs, _part_1_solution: String) -> Result<String> {
        let mut stacks = input.0.clone();

        // apply instrs
        for (n, from, to) in &input.1 {
            let mut crates = vec![];
            for _ in 0..*n {
                let cr = stacks[*from].pop().unwrap();
                crates.push(cr);
            }
            crates.reverse();
            stacks[*to].extend(crates);
        }

        // get tops
        let mut res = String::from("");
        for stack in stacks {
            res.push(*stack.last().unwrap());
        }

        Ok(res.to_string())
    }
}

pub fn day05(input: &str) -> Result<f32> {
    solve_linear::<Day5Solution, _, _, _>(input)
}

#[cfg(test)]
mod tests {
    use super::Day5Solution;
    use crate::utils::solver_types::SolutionLinear;

    #[test]
    fn test_answer() {
        // note: ugly format so we don't lead with a \n
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

        let p1_expected = "CMZ".to_string();
        let p2_expected = "MCD".to_string();

        let mut loaded = Day5Solution::load(input).unwrap();

        let p1 = Day5Solution::part1(&mut loaded).unwrap();
        assert_eq!(p1_expected, p1);

        let p2 = Day5Solution::part2(&mut loaded, p1.clone()).unwrap();

        assert_eq!(p2_expected, p2);
    }
}
