use crate::utils::{
    grid::Grid,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use itertools::Itertools;

//not yet implemented
pub struct Day10Solution {}

fn execute_record(input: &[isize]) -> isize {
    let mut reg = 1;

    let mut next_read = 19;
    let mut signal = 0;

    for (i, add) in input.iter().enumerate() {
        match i {
            220 => break,
            v if v == next_read => {
                signal += (1 + next_read) as isize * reg; // lmao off by ones
                next_read += 40;
            }
            _ => (),
        }
        reg += add;
    }

    signal
}

fn draw(input: &[isize]) -> Vec<Vec<isize>> {
    let mut reg = 1;

    let mut drawn = vec![];

    for (i, add) in (0..241).zip(input.iter()) {
        let cursor = i % 40;

        let distance: isize = cursor - reg;
        if distance.abs() < 2 {
            drawn.push(vec![cursor, i / 40])
        }

        reg += add;
    }

    drawn
}

impl SolutionLinear<Vec<isize>, isize, String> for Day10Solution {
    fn load(input: &str) -> Result<Vec<isize>> {
        Ok(input
            .lines()
            .flat_map(|l| match l.split_once(' ') {
                None => vec![0],
                Some(("addx", v)) => {
                    let v = v.parse::<isize>().unwrap();
                    vec![0, v]
                }
                _ => unreachable!(),
            })
            .collect())
    }

    fn part1(input: &mut Vec<isize>) -> Result<isize> {
        Ok(execute_record(input))
    }

    fn part2(input: &mut Vec<isize>, _part_1_solution: isize) -> Result<String> {
        let to_draw = draw(input);

        let g = Grid::<bool, 2>::from(to_draw.into_iter().map(|p| (p, true)).collect_vec());

        Ok(g.print(|c| if c { '█' } else { ' ' }))
    }
}

pub fn day10(input: &str) -> Result<f32> {
    solve_linear::<Day10Solution, _, _, _>(input)
}

#[cfg(test)]
mod tests {
    use super::Day10Solution;
    use crate::utils::solver_types::SolutionLinear;

    #[test]
    fn test_answer() {
        // note: ugly format so we don't lead with a \n
        let input = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;

        let p1_expected = 13140;
        let p2_expected = r#"
██  ██  ██  ██  ██  ██  ██  ██  ██  ██  
███   ███   ███   ███   ███   ███   ███ 
████    ████    ████    ████    ████    
█████     █████     █████     █████     
██████      ██████      ██████      ████
███████       ███████       ███████     
"#;

        let mut loaded = Day10Solution::load(input).unwrap();

        let p1 = Day10Solution::part1(&mut loaded).unwrap();
        assert_eq!(p1_expected, p1);

        let p2 = Day10Solution::part2(&mut loaded, p1).unwrap();
        println!("{}", p2);
        assert_eq!(p2_expected, p2);
    }
}
