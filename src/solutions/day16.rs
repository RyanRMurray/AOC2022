use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Deref;
use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    ops::IndexMut,
};

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"Valve (.+) has flow rate=(\d+); tunnels? leads? to valves? (.+)").unwrap();
}

pub struct Day16Solution {}

fn distance_between(graph: &HashMap<&str, Vec<&str>>, start: &str, end: &str) -> usize {
    let mut to_visit = vec![start];
    let mut distance = 0;

    loop {
        if to_visit.contains(&end) {
            return distance;
        }
        let mut next: Vec<&str> = vec![];
        for v in to_visit {
            next.extend(graph.get(v).unwrap().iter().map(Deref::deref));
        }
        distance += 1;
        to_visit = next;
    }
}

fn dfs(input: &Input, start: usize, visited: HashSet<usize>, remaining: usize) -> usize {
    let mut max_seen = 0;

    for (next, distance) in input.distances.get(start).unwrap().iter().enumerate() {
        if visited.contains(&next) || distance + 1 > remaining {
            continue;
        }

        let flow = input.flow_rates.get(&next).unwrap();

        let mut next_visited = visited.clone();
        next_visited.extend([next]);
        let utility = (flow * (remaining - distance - 1))
            + dfs(input, next, next_visited, remaining - distance - 1);

        max_seen = max(max_seen, utility);
    }

    max_seen
}

fn to_signature(v: &Vec<usize>) -> u16 {
    let mut signature = 0;

    for x in v {
        signature |= 1 << x;
    }

    signature
}

#[derive(Debug)]
struct Input {
    distances: Vec<Vec<usize>>,
    flow_rates: HashMap<usize, usize>,
}

impl SolutionLinear<Input, usize, usize> for Day16Solution {
    fn load(input: &str) -> Result<Input> {
        let mut str_map = HashMap::from([("AA", 0)]);
        let mut important = vec!["AA"];
        let mut flow_rates = HashMap::from([(0, 0)]);
        let mut adjs = HashMap::new();

        for l in input.lines() {
            let ms = RE.captures(l).unwrap();

            let node = ms.get(1).unwrap().as_str();

            let flow = ms.get(2).unwrap().as_str().parse::<usize>().unwrap();

            if flow > 0 {
                str_map.insert(node, important.len());
                flow_rates.insert(important.len(), flow);
                important.push(node);
            }

            adjs.insert(node, ms.get(3).unwrap().as_str().split(", ").collect_vec());
        }

        // generate matrix of distances
        let mut distances: Vec<Vec<usize>> = vec![vec![0; important.len()]; important.len()];

        for a in &important {
            for b in &important {
                if a == b {
                    continue;
                }
                let i_a = *str_map.get(a).unwrap();
                let i_b = *str_map.get(b).unwrap();
                let dist = distance_between(&adjs, a, b);
                *distances.get_mut(i_a).unwrap().index_mut(i_b) = dist;
            }
        }

        Ok(Input {
            distances,
            flow_rates,
        })
    }

    fn part1(input: &mut Input) -> Result<usize> {
        Ok(dfs(input, 0, HashSet::new(), 30))
    }

    fn part2(input: &mut Input, _part_1_solution: usize) -> Result<usize> {
        let mut max_seen = 0;
        let mut seen = HashSet::new();
        let all: Vec<usize> = (0..input.flow_rates.len()).collect();

        let powerset: Vec<Vec<usize>> = input
            .flow_rates
            .keys()
            .powerset()
            .map(|set| set.into_iter().copied().sorted().collect_vec())
            .filter(|v| match v.first() {
                None => false,
                Some(f) => *f == 0,
            })
            .collect();

        for p in powerset {
            seen.insert(to_signature(&p));
            let mut other = vec![0];
            other.extend(all.iter().filter(|v| !p.contains(v)));

            if seen.contains(&to_signature(&other)) {
                break;
            }

            max_seen = max(
                max_seen,
                dfs(input, 0, HashSet::from_iter(p), 26)
                    + dfs(input, 0, HashSet::from_iter(other), 26),
            );
        }

        Ok(max_seen)
    }
}

pub fn day16(input: &str) -> Result<f32> {
    solve_linear::<Day16Solution, _, _, _>(input)
}

#[cfg(test)]
mod tests {
    use super::Day16Solution;
    use crate::utils::solver_types::SolutionLinear;

    #[test]
    fn test_answer() {
        // note: ugly format so we don't lead with a \n
        let input = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;

        let p1_expected = 1651;
        let p2_expected = 1707;

        let mut loaded = Day16Solution::load(input).unwrap();
        let p1 = Day16Solution::part1(&mut loaded).unwrap();
        assert_eq!(p1_expected, p1);

        let p2 = Day16Solution::part2(&mut loaded, p1).unwrap();

        assert_eq!(p2_expected, p2);
    }
}
