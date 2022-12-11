use std::collections::VecDeque;

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
pub struct Day11Solution {}

#[derive(Default, Debug, Clone)]
enum Op {
    #[default]
    MulSelf,
    AddSelf,
    Mul(usize),
    Add(usize),
}
#[derive(Default, Debug, Clone)]
struct Monkeys {
    pub inventories: Vec<VecDeque<usize>>,
    pub operations: Vec<Op>,
    pub tests: Vec<(usize, usize, usize)>,
    pub inspections: Vec<usize>,
}

fn do_test((div, a, b): &(usize, usize, usize), item: &usize) -> usize {
    if item % div == 0 {
        *a
    } else {
        *b
    }
}

fn apply_monkey(
    input: usize,
    op: &Op,
    test: &(usize, usize, usize),
    lowest_common_monkeple: Option<usize>,
) -> (usize, usize) {
    let new_value = match op {
        Op::MulSelf => input * input,
        Op::AddSelf => input + input,
        Op::Mul(v) => input * v,
        Op::Add(v) => input + v,
    };

    let div_val = match lowest_common_monkeple {
        None => new_value / 3,
        Some(v) => new_value % v,
    };

    let new_owner = do_test(test, &div_val);

    (new_owner, div_val)
}
fn do_round(monkeys: &mut Monkeys, lowest_common_monkeple: Option<usize>) {
    for i in 0..monkeys.inventories.len() {
        while !monkeys.inventories.get(i).unwrap().is_empty() {
            let item = monkeys.inventories.get_mut(i).unwrap().pop_front().unwrap();
            let (new_owner, new_value) = apply_monkey(
                item,
                monkeys.operations.get(i).unwrap(),
                monkeys.tests.get(i).unwrap(),
                lowest_common_monkeple,
            );
            monkeys
                .inventories
                .get_mut(new_owner)
                .unwrap()
                .push_back(new_value);

            *monkeys.inspections.get_mut(i).unwrap() += 1;
        }
    }
}

impl SolutionLinear<Monkeys, usize, usize> for Day11Solution {
    fn load(input: &str) -> Result<Monkeys> {
        let monkeys = input.split("\n\n");

        let mut result = Monkeys::default();

        for m in monkeys {
            let ls: [&str; 6] = m.lines().collect::<Vec<_>>().try_into().unwrap();
            // get inv
            let inv = ls[1]
                .split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .map(|v| v.parse::<usize>().unwrap())
                .collect();
            result.inventories.push(inv);

            // get op
            let op = ls[2].split(' ').nth(6).unwrap();
            let val: Result<usize, _> = ls[2].split(' ').nth(7).unwrap().parse();
            let operation = match (op, val) {
                ("+", Err(_)) => Op::AddSelf,
                ("*", Err(_)) => Op::MulSelf,
                ("+", Ok(val)) => Op::Add(val),
                ("*", Ok(val)) => Op::Mul(val),
                _ => unreachable!(),
            };
            result.operations.push(operation);

            // get test
            let test = (
                ls[3].split(' ').last().unwrap().parse().unwrap(),
                ls[4].split(' ').last().unwrap().parse().unwrap(),
                ls[5].split(' ').last().unwrap().parse().unwrap(),
            );
            result.tests.push(test);

            result.inspections.push(0);
        }
        Ok(result)
    }

    fn part1(input: &mut Monkeys) -> Result<usize> {
        let mut monkeys = input.clone();
        for _ in 0..20 {
            do_round(&mut monkeys, None);
        }

        monkeys.inspections.sort();
        Ok(monkeys.inspections.pop().unwrap() * monkeys.inspections.pop().unwrap())
    }

    fn part2(input: &mut Monkeys, _part_1_solution: usize) -> Result<usize> {
        // find lowest common monkeple
        let lcm: usize = input.tests.iter().map(|t| t.0).product();

        for _ in 0..10000 {
            do_round(input, Some(lcm));
        }

        let mut results = input.inspections.clone();
        results.sort();
        Ok(results.pop().unwrap() * results.pop().unwrap())
    }
}

pub fn day11(input: &str) -> Result<f32> {
    solve_linear::<Day11Solution, _, _, _>(input)
}

#[cfg(test)]
mod tests {
    use super::Day11Solution;
    use crate::utils::solver_types::SolutionLinear;

    #[test]
    fn test_answer() {
        // note: ugly format so we don't lead with a \n
        let input = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

        let p1_expected = 10605;
        let p2_expected = 2713310158;

        let mut loaded = Day11Solution::load(input).unwrap();
        let p1 = Day11Solution::part1(&mut loaded).unwrap();
        assert_eq!(p1_expected, p1);

        let p2 = Day11Solution::part2(&mut loaded, p1).unwrap();

        assert_eq!(p2_expected, p2);
    }
}
