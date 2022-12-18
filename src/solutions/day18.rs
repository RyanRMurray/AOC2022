use std::collections::HashSet;

use crate::utils::grid::Grid;
use crate::utils::point::Pt;
use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::Itertools;

pub struct Day18Solution {}

impl SolutionLinear<Grid<bool, 3>, usize, usize> for Day18Solution {
    fn load(input: &str) -> Result<Grid<bool, 3>> {
        Ok(Grid::from(
            input
                .lines()
                .map(|l| {
                    let mut coords = l.split(',');
                    (
                        Pt([
                            coords.next().unwrap().parse().unwrap(),
                            coords.next().unwrap().parse().unwrap(),
                            coords.next().unwrap().parse().unwrap(),
                        ]),
                        true,
                    )
                })
                .collect_vec(),
        ))
    }

    fn part1(input: &mut Grid<bool, 3>) -> Result<usize> {
        Ok(input
            .grid
            .keys()
            .map(|coord| {
                input
                    .card_offsets
                    .iter()
                    .filter(|off| !input.grid.contains_key(&(coord + off)))
                    .count()
            })
            .sum())
    }

    fn part2(input: &mut Grid<bool, 3>, _part_1_solution: usize) -> Result<usize> {
        let mut adjs = HashSet::new();

        let visible_count: usize = input
            .grid
            .keys()
            .map(|coord| {
                input
                    .card_offsets
                    .iter()
                    .filter(|off| match input.grid.contains_key(&(coord + off)) {
                        false => {
                            adjs.insert(coord + off);
                            true
                        }
                        true => false,
                    })
                    .count()
            })
            .sum();

        // flood-search a bounding box for the shape
        let ([min_x, min_y, min_z], [max_x, max_y, max_z]) = input.bounds();

        let mut visited = HashSet::new();
        let mut to_visit = vec![Pt([min_x - 1, min_y - 1, min_z])];

        while !to_visit.is_empty() {
            let visiting = to_visit.pop().unwrap();
            for off in Pt::<3>::card_offsets() {
                let n = visiting + off;
                let Pt([nx, ny, nz]) = n;

                if visited.contains(&n)
                    || input.grid.contains_key(&n)
                    || nx < min_x - 1
                    || nx > max_x + 1
                    || ny < min_y - 1
                    || ny > max_y + 1
                    || nz < min_z - 1
                    || nz > max_z + 1
                {
                    continue;
                }

                to_visit.push(n);
            }
            visited.insert(visiting);
        }

        // get number of faces only exposed in air bubbles
        let bubble_faces: usize = adjs
            .difference(&visited)
            .map(|pos| {
                Pt::<3>::card_offsets()
                    .iter()
                    .filter(|off| input.grid.contains_key(&(pos + off)))
                    .count()
            })
            .sum();

        Ok(visible_count - bubble_faces)
    }
}

pub fn day18(input: &str) -> Result<f32> {
    solve_linear::<Day18Solution, _, _, _>(input)
}

#[cfg(test)]
mod tests {
    use super::Day18Solution;
    use crate::utils::solver_types::SolutionLinear;

    #[test]
    fn test_answer() {
        let input = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#;

        let p1_expected = 64;
        let p2_expected = 58;

        let mut loaded = Day18Solution::load(input).unwrap();
        let p1 = Day18Solution::part1(&mut loaded).unwrap();
        assert_eq!(p1_expected, p1);

        let p2 = Day18Solution::part2(&mut loaded, p1).unwrap();

        assert_eq!(p2_expected, p2);
    }
}
