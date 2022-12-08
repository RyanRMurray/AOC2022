use crate::utils::{
    grid::Grid,
    load_input::load_2d_grid,
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use itertools::Itertools;

//not yet implemented
pub struct Day8Solution {}

type TreeGr = Grid<u8, 2>;

fn visible_from(
    grid: &TreeGr,
    mut visible: Grid<bool, 2>,
    start: Pt<2>,
    dir: Pt<2>,
    steps: isize,
) -> Grid<bool, 2> {
    let mut last_highest = grid.get_def(&start);
    let mut pos = start;
    visible.grid.insert(pos, true);

    for _ in 0..steps {
        let next_pos = pos + dir;
        let next = grid.get_def(&next_pos);
        if next > last_highest {
            last_highest = next;
            visible.grid.insert(next_pos, true);
        }
        pos = next_pos;
    }

    visible
}

fn get_trees_in_dir(grid: &TreeGr, start: Pt<2>, dir: Pt<2>, steps: isize) -> usize {
    let mut ptr = start;
    let max = grid.get_def(&start);
    let mut counter = 0;

    for _ in 0..steps {
        ptr = ptr + dir;
        let new_tree = grid.get_def(&ptr);
        counter += 1;
        if new_tree >= max {
            break;
        }
    }

    counter
}

impl SolutionLinear<TreeGr, usize, usize> for Day8Solution {
    fn load(input: &str) -> Result<TreeGr> {
        Ok(load_2d_grid(input, |v| v.to_digit(10).unwrap() as u8))
    }

    fn part1(input: &mut TreeGr) -> Result<usize> {
        let (_, [max_x, max_y]) = input.bounds();

        let mut visible = Grid::default();

        for y in 0..max_y + 1 {
            visible = visible_from(input, visible, Pt([0, y]), Pt([1, 0]), max_y);
            visible = visible_from(input, visible, Pt([max_x, y]), Pt([-1, 0]), max_y);
        }

        for x in 1..max_x {
            visible = visible_from(input, visible, Pt([x, 0]), Pt([0, 1]), max_y);
            visible = visible_from(input, visible, Pt([x, max_y]), Pt([0, -1]), max_y);
        }

        Ok(visible.grid.iter().filter(|(_, t)| **t).count())
    }

    fn part2(input: &mut TreeGr, _part_1_solution: usize) -> Result<usize> {
        let (_, [max_x, max_y]) = input.bounds();

        let mut max = 0;

        for (x, y) in (1..max_x).cartesian_product(1..max_y) {
            let mut score = 1;
            score *= get_trees_in_dir(input, Pt([x, y]), Pt([-1, 0]), x);
            score *= get_trees_in_dir(input, Pt([x, y]), Pt([1, 0]), max_x - x);
            score *= get_trees_in_dir(input, Pt([x, y]), Pt([0, -1]), y);
            score *= get_trees_in_dir(input, Pt([x, y]), Pt([0, 1]), max_y - y);

            if score > max {
                max = score;
            }
        }
        Ok(max)
    }
}

pub fn day08(input: &str) -> Result<f32> {
    solve_linear::<Day8Solution, _, _, _>(input)
}

#[cfg(test)]
mod tests {
    use super::Day8Solution;
    use crate::utils::solver_types::SolutionLinear;

    #[test]
    fn test_answer() {
        // note: ugly format so we don't lead with a \n
        let input = r#"30373
25512
65332
33549
35390"#;

        let p1_expected = 21;
        let p2_expected = 8;

        let mut loaded = Day8Solution::load(input).unwrap();

        let p1 = Day8Solution::part1(&mut loaded).unwrap();
        assert_eq!(p1_expected, p1);

        let p2 = Day8Solution::part2(&mut loaded, p1).unwrap();

        assert_eq!(p2_expected, p2);
    }
}
