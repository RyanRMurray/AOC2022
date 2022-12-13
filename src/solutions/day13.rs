use std::{cmp::Ordering, iter::Peekable, str::Chars};

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::{EitherOrBoth, Itertools};

//not yet implemented
pub struct Day13Solution {}

#[derive(Debug, Clone)]
enum Pa {
    Num(u32),
    List(Vec<Pa>),
}

impl Pa {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Pa::Num(a), Pa::Num(b)) => a.cmp(b),
            (Pa::List(l1), Pa::List(l2)) => {
                for pair in l1.iter().zip_longest(l2.iter()) {
                    match pair {
                        EitherOrBoth::Left(_) => return Ordering::Greater,
                        EitherOrBoth::Right(_) => return Ordering::Less,
                        EitherOrBoth::Both(a, b) => {
                            let c = a.cmp(b);
                            if c == Ordering::Equal {
                                continue;
                            }
                            return c;
                        }
                    }
                }
                Ordering::Equal
            }
            (Pa::Num(a), Pa::List(b)) => Pa::List(vec![Pa::Num(*a)]).cmp(&Pa::List(b.to_vec())),
            (Pa::List(a), Pa::Num(b)) => Pa::List(a.to_vec()).cmp(&Pa::List(vec![Pa::Num(*b)])),
        }
    }
}

fn to_pa(packet: &mut Peekable<Chars>) -> Vec<Pa> {
    let mut result = vec![];

    while packet.peek().is_some() {
        match packet.next().unwrap() {
            ',' => (),
            '[' => result.push(Pa::List(to_pa(packet))),
            ']' => break,
            val => {
                let next = packet.peek().and_then(|v| v.to_digit(10));

                match next {
                    None => result.push(Pa::Num(val.to_digit(10).unwrap())),
                    _ => result.push(Pa::Num(
                        format!("{}{}", val, packet.next().unwrap())
                            .parse()
                            .unwrap(),
                    )),
                }
            }
        }
    }

    result
}

impl SolutionLinear<Vec<(Pa, Pa)>, usize, usize> for Day13Solution {
    fn load(input: &str) -> Result<Vec<(Pa, Pa)>> {
        Ok(input
            .split("\n\n")
            .map(|pair| {
                let (l, r) = pair.split_once('\n').unwrap();

                (
                    Pa::List(to_pa(&mut l.chars().peekable())),
                    Pa::List(to_pa(&mut r.chars().peekable())),
                )
            })
            .collect())
    }

    fn part1(input: &mut Vec<(Pa, Pa)>) -> Result<usize> {
        Ok(input
            .iter()
            .enumerate()
            .map(
                |(i, (x, y))| {
                    if x.cmp(y) == Ordering::Less {
                        i + 1
                    } else {
                        0
                    }
                },
            )
            .sum())
    }

    fn part2(input: &mut Vec<(Pa, Pa)>, _part_1_solution: usize) -> Result<usize> {
        let mut all_packets: Vec<Pa> = input
            .iter()
            .flat_map(|(x, y)| vec![x.clone(), y.clone()])
            .collect();
        let mut result = 1;

        let div_a = Pa::List(vec![Pa::List(vec![Pa::Num(2)])]);
        let div_b = Pa::List(vec![Pa::List(vec![Pa::Num(6)])]);

        all_packets.push(div_a.clone());
        all_packets.push(div_b.clone());

        all_packets.sort_by(|a, b| a.cmp(b));

        for (i, p) in all_packets.iter().enumerate() {
            match p {
                p if p.cmp(&div_a) == Ordering::Equal => result *= i + 1,
                p if p.cmp(&div_b) == Ordering::Equal => result *= i + 1,
                _ => (),
            }
        }

        Ok(result)
    }
}

pub fn day13(input: &str) -> Result<f32> {
    solve_linear::<Day13Solution, _, _, _>(input)
}

#[cfg(test)]
mod tests {
    use super::Day13Solution;
    use crate::utils::solver_types::SolutionLinear;

    #[test]
    fn test_answer() {
        // note: ugly format so we don't lead with a \n
        let input = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;

        let p1_expected = 13;
        let p2_expected = 140;

        let mut loaded = Day13Solution::load(input).unwrap();

        let p1 = Day13Solution::part1(&mut loaded).unwrap();
        assert_eq!(p1_expected, p1);

        let p2 = Day13Solution::part2(&mut loaded, p1).unwrap();

        assert_eq!(p2_expected, p2);
    }
}
