use std::collections::HashSet;

use crate::utils::{
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
pub struct Day9Solution {}

type Instr = (Pt<2>, usize); //direction, repeats

fn to_instr(line: &str) -> Instr {
    let (dir, reps) = line.split_once(' ').expect("Check input");

    let dir_pt = match dir {
        "U" => Pt([0, -1]),
        "D" => Pt([0, 1]),
        "L" => Pt([-1, 0]),
        "R" => Pt([1, 0]),
        _ => unreachable!(),
    };
    (dir_pt, reps.parse().expect("Check input"))
}

/// return the position tail would have to go to 'catch up' to head.
fn catch_up(tail: &Pt<2>, head: &Pt<2>) -> Pt<2> {
    let Pt([x, y]) = head - tail;

    match (x.abs(), y.abs()) {
        (0, 0) | (1, 1) | (1, 0) | (0, 1) => *tail,
        (a, b) if a > b => *head - Pt([x.signum(), 0]),
        (a, b) if a == b => *head - Pt([x.signum(), y.signum()]),
        _ => *head - Pt([0, y.signum()]),
    }
}

impl SolutionLinear<Vec<Instr>, usize, usize> for Day9Solution {
    fn load(input: &str) -> Result<Vec<Instr>> {
        Ok(input.lines().map(to_instr).collect())
    }

    fn part1(input: &mut Vec<Instr>) -> Result<usize> {
        let mut head = Pt::<2>::default();
        let mut tail = Pt::<2>::default();
        let mut tail_visited = HashSet::new();

        for (dir, reps) in input.iter() {
            for _ in 0..*reps {
                head += *dir;
                tail = catch_up(&tail, &head);
                tail_visited.insert(tail);
            }
        }
        Ok(tail_visited.len())
    }

    fn part2(input: &mut Vec<Instr>, _part_1_solution: usize) -> Result<usize> {
        let mut rope = vec![Pt::<2>::default(); 10];

        let mut tail_visited = HashSet::new();

        for (dir, reps) in input.iter() {
            for _ in 0..*reps {
                rope[0] += *dir;
                for seg in 1..10 {
                    let move_to = catch_up(&rope[seg], &rope[seg - 1]);

                    rope[seg] = move_to;
                }
                tail_visited.insert(rope[9]);
            }
        }

        Ok(tail_visited.len())
    }
}

pub fn day09(input: &str) -> Result<f32> {
    solve_linear::<Day9Solution, _, _, _>(input)
}

#[cfg(test)]
mod tests {
    use super::Day9Solution;
    use crate::utils::solver_types::SolutionLinear;

    #[test]
    fn test_answer() {
        // note: ugly format so we don't lead with a \n
        let input = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

        let p1_expected = 13;
        let p2_expected = 1;

        let mut loaded = Day9Solution::load(input).unwrap();
        let p1 = Day9Solution::part1(&mut loaded).unwrap();
        assert_eq!(p1_expected, p1);

        let p2 = Day9Solution::part2(&mut loaded, p1).unwrap();

        assert_eq!(p2_expected, p2);
    }

    #[test]
    fn test_lomg_boy() {
        // note: ugly format so we don't lead with a \n
        let input = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

        let p2_expected = 36;

        let mut loaded = Day9Solution::load(input).unwrap();
        let p2 = Day9Solution::part2(&mut loaded, 1).unwrap();

        assert_eq!(p2_expected, p2);
    }
}
