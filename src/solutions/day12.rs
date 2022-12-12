use std::collections::{HashMap, HashSet};

use crate::utils::{
    grid::Grid,
    load_input::load_2d_grid,
    point::Pt,
    solver_types::{solve_simultaneous, SolutionSimultaneous},
};
use anyhow::Result;
use itertools::Itertools;
use lazy_static::lazy_static;

//not yet implemented
pub struct Day12Solution {}

#[derive(Default, Debug)]
struct HeightGraph {
    edges: HashMap<Pt<2>, Vec<Pt<2>>>,
    start: Pt<2>,
    end: Pt<2>,
    all_starts: Vec<Pt<2>>,
}

lazy_static! {
    static ref OFFSETS: Vec<Pt<2>> = Pt::<2>::card_offsets().into_iter().collect_vec();
}

fn generate_distances(graph: &HeightGraph, start: &Pt<2>) -> HashMap<Pt<2>, usize> {
    let mut distances = HashMap::new();
    let mut visited: HashSet<Pt<2>> = HashSet::default();
    let mut to_visit: HashSet<Pt<2>> = HashSet::from([*start]);
    let mut steps = 0;

    loop {
        visited.extend(to_visit.iter());
        let mut next_visit: HashSet<Pt<2>> = HashSet::new();
        for pt in to_visit.iter() {
            distances.insert(*pt, steps);

            if let Some(ns) = graph.edges.get(pt) {
                next_visit.extend(ns);
            }
        }
        to_visit = next_visit
            .into_iter()
            .filter(|pt| !visited.contains(pt))
            .collect();
        if to_visit.is_empty() {
            break;
        }
        steps += 1;
    }
    distances
}

impl SolutionSimultaneous<HeightGraph, usize, usize> for Day12Solution {
    fn load(input: &str) -> Result<HeightGraph> {
        let grid: Grid<u8, 2> = load_2d_grid(input, |v| match v {
            'S' => 0,
            'E' => 27,
            c => c as u8 - 96,
        });

        // form graph
        let mut graph = HeightGraph::default();

        for pt in grid.grid.keys() {
            let mut v = grid.get_def(pt);
            if v == 1 {
                graph.all_starts.push(*pt);
            }
            if v == 0 {
                graph.start = *pt;
                v += 1;
            }
            if v == 27 {
                graph.end = *pt;
                v -= 1;
            }
            for neighbour in OFFSETS
                .iter()
                .map(|off| (pt + off))
                .filter(|pos| grid.grid.contains_key(pos))
            {
                if grid.get_def(&neighbour) >= v - 1 {
                    graph.edges.entry(*pt).or_default().push(neighbour);
                }
            }
        }

        Ok(graph)
    }

    fn solve(input: HeightGraph) -> Result<(usize, usize)> {
        let distances = generate_distances(&input, &input.end);

        Ok((
            *distances.get(&input.start).unwrap(),
            *input
                .all_starts
                .iter()
                .map(|a| distances.get(a).unwrap_or(&usize::MAX))
                .min()
                .unwrap(),
        ))
    }
}

pub fn day12(input: &str) -> Result<f32> {
    solve_simultaneous::<Day12Solution, _, _, _>(input)
}

#[cfg(test)]
mod tests {
    use super::Day12Solution;
    use crate::utils::solver_types::SolutionSimultaneous;

    #[test]
    fn test_answer() {
        // note: ugly format so we don't lead with a \n
        let input = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

        let p1_expected = 31;
        let p2_expected = 29;

        let loaded = Day12Solution::load(input).unwrap();
        let (p1, p2) = Day12Solution::solve(loaded).unwrap();
        assert_eq!(p1_expected, p1);
        assert_eq!(p2_expected, p2);
    }
}
