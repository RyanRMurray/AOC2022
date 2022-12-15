use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

use crate::utils::{
    point::Pt,
    solver_types::{solve_linear, SolutionLinear},
};
use anyhow::Result;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(
        r"Sensor at x=([-\d]+), y=([-\d]+): closest beacon is at x=([-\d]+), y=([-\d]+)"
    )
    .unwrap();
}

pub struct Day15Solution {}

struct Input {
    target: isize,
    beacons: HashSet<Pt<2>>,
    covered: HashMap<isize, Vec<(isize, isize)>>, // y co-ord -> bounds covered by beacons
}

fn manhattan(Pt([x1, y1]): &Pt<2>, Pt([x2, y2]): &Pt<2>) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}

impl SolutionLinear<Input, usize, isize> for Day15Solution {
    fn load(input: &str) -> Result<Input> {
        let sen_beacons = input
            .lines()
            .map(|l| {
                let caps = RE.captures(l).unwrap();

                (
                    Pt([
                        caps.get(1).unwrap().as_str().parse().unwrap(),
                        caps.get(2).unwrap().as_str().parse().unwrap(),
                    ]),
                    Pt([
                        caps.get(3).unwrap().as_str().parse().unwrap(),
                        caps.get(4).unwrap().as_str().parse().unwrap(),
                    ]),
                )
            })
            .collect_vec();

        let beacons = sen_beacons.iter().map(|(_, pt)| *pt).collect();

        let mut covered: HashMap<isize, Vec<(isize, isize)>> = HashMap::new();

        for (sen, beacon) in sen_beacons.iter() {
            let distance = manhattan(sen, beacon);
            for y in sen.0[1] - distance..sen.0[1] + distance + 1 {
                let y_distance = (sen.0[1] - y).abs();

                covered.entry(y).or_default().push((
                    // x-left
                    sen.0[0] - (distance - y_distance),
                    // x-right
                    sen.0[0] + (distance - y_distance),
                ))
            }
        }

        for (_, row) in covered.iter_mut() {
            row.sort()
        }

        Ok(Input {
            target: 2_000_000,
            beacons,
            covered,
        })
    }

    fn part1(input: &mut Input) -> Result<usize> {
        // get contigous area covered on row 10
        let mut target_row = HashSet::new();
        let covered = input.covered.get(&input.target).unwrap();

        for (a, b) in covered {
            for x in *a..*b + 1 {
                target_row.insert(Pt([x, input.target]));
            }
        }

        Ok(target_row.difference(&input.beacons).count())
    }

    fn part2(input: &mut Input, _part_1_solution: usize) -> Result<isize> {
        // for each row, find the x not covered between 0 and target*2
        for y in 0..input.target * 2 {
            let mut last = 0;
            for (a, b) in input.covered.get(&y).unwrap() {
                if last < *a {
                    return Ok((a - 1) * 4_000_000 + y);
                }

                last = max(last, *b);
            }
        }

        unreachable!()
    }
}

pub fn day15(input: &str) -> Result<f32> {
    solve_linear::<Day15Solution, _, _, _>(input)
}

#[cfg(test)]
mod tests {
    use super::Day15Solution;
    use crate::utils::solver_types::SolutionLinear;

    #[test]
    fn test_answer() {
        // note: ugly format so we don't lead with a \n
        let input = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;

        let p1_expected = 26;
        let p2_expected = 56000011;

        let mut loaded = Day15Solution::load(input).unwrap();
        loaded.target = 10; // note: overload for test case

        let p1 = Day15Solution::part1(&mut loaded).unwrap();
        assert_eq!(p1_expected, p1);

        let p2 = Day15Solution::part2(&mut loaded, p1).unwrap();

        assert_eq!(p2_expected, p2);
    }
}
