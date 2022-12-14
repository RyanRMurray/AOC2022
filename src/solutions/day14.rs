use crate::utils::{
    grid::Grid,
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use itertools::Itertools;

//not yet implemented
pub struct Day14Solution {}

#[derive(Copy, Clone, Default)]
enum Space {
    #[default]
    Open,
    Sand,
    Rock,
}

const OFFSETS: [Pt<2>; 3] = [Pt([0, 1]), Pt([-1, 1]), Pt([1, 1])]; // spaces to check when falling

fn do_sand(
    grid: &Grid<Space, 2>,
    threshold: &isize,
    floor: bool,
    fall_stack: &mut Vec<Pt<2>>,
) -> Option<Pt<2>> {
    // get start
    let mut sand;
    loop {
        match fall_stack.pop() {
            None => return None,
            Some(pos) if !grid.grid.contains_key(&pos) => {
                sand = pos;
                break;
            }
            _ => continue,
        }
    }

    // settle grain
    'fall: loop {
        if sand.0[1] >= *threshold && !floor {
            return None;
        }

        for off in OFFSETS.iter() {
            let next = sand + *off;
            if !grid.grid.contains_key(&next) && next.0[1] <= *threshold {
                fall_stack.push(sand);
                sand += *off;
                continue 'fall;
            }
        }

        // solid ground below, settle
        return Some(sand);
    }
}

impl SolutionLinear<Grid<Space, 2>, usize, usize> for Day14Solution {
    fn load(input: &str) -> Result<Grid<Space, 2>> {
        let rocks = input
            .lines()
            .flat_map(|l| {
                let mut ends = l.split(" -> ").map(|pos| {
                    let (x, y) = pos.split_once(',').unwrap();
                    Pt([x.parse().unwrap(), y.parse().unwrap()])
                });

                let mut last = ends.next().unwrap();
                let mut pts = vec![];

                for Pt([x, y]) in ends {
                    if x != last.0[0] {
                        let mut xs = [x, last.0[0]];
                        xs.sort();
                        for xx in xs[0]..xs[1] + 1 {
                            pts.push(Pt([xx, y]));
                        }
                    } else {
                        let mut ys = [y, last.0[1]];
                        ys.sort();
                        for yy in ys[0]..ys[1] + 1 {
                            pts.push(Pt([x, yy]));
                        }
                    }
                    last = Pt([x, y])
                }
                pts.into_iter().map(|p| (p, Space::Rock))
            })
            .collect_vec();

        Ok(Grid::from(rocks))
    }

    fn part1(input: &mut Grid<Space, 2>) -> Result<usize> {
        let threshold = input.bounds().1[1]; // maximum y distance - beyond this is the void
        let mut settled = 0;

        let mut fall_stack = vec![Pt([500, 0])];

        loop {
            match do_sand(input, &threshold, false, &mut fall_stack) {
                None => return Ok(settled),
                Some(pos) => {
                    settled += 1;
                    input.grid.insert(pos, Space::Sand);
                }
            }
        }
    }

    fn part2(input: &mut Grid<Space, 2>, part_1_solution: usize) -> Result<usize> {
        let threshold = input.bounds().1[1] + 1;
        let mut settled = part_1_solution;
        let mut fall_stack = vec![Pt([500, 0])];

        loop {
            match do_sand(input, &threshold, true, &mut fall_stack) {
                None => return Ok(settled),
                Some(pos) => {
                    settled += 1;
                    input.grid.insert(pos, Space::Sand);
                }
            }
        }
    }
}

pub fn day14(input: &str) -> Result<f32> {
    solve_linear::<Day14Solution, _, _, _>(input)
}

#[allow(dead_code)]
fn print_maze(grid: &Grid<Space, 2>) {
    println!(
        "{}",
        grid.print(|c| match c {
            Space::Rock => '#',
            Space::Sand => 'O',
            Space::Open => '.',
        })
    );
}

#[cfg(test)]
mod tests {
    use super::Day14Solution;
    use crate::utils::solver_types::SolutionLinear;

    #[test]
    fn test_answer() {
        // note: ugly format so we don't lead with a \n
        let input = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

        let p1_expected = 24;
        let p2_expected = 93;

        let mut loaded = Day14Solution::load(input).unwrap();

        let p1 = Day14Solution::part1(&mut loaded).unwrap();
        assert_eq!(p1_expected, p1);
        let p2 = Day14Solution::part2(&mut loaded, p1).unwrap();

        assert_eq!(p2_expected, p2);
    }
}
