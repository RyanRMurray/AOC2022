use std::collections::{HashMap, VecDeque};

use crate::utils::solver_types::{solve_linear, SolutionLinear};
use anyhow::Result;
use itertools::Itertools;

//not yet implemented
pub struct Day17Solution {}

type Shape = [u8; 4];

const FLAT: Shape = [0b0000_0000, 0b0000_0000, 0b0000_0000, 0b0001_1110];

const CROSS: Shape = [0b0000_0000, 0b0000_1000, 0b0001_1100, 0b0000_1000];

const CORNER: Shape = [0b0000_0000, 0b0000_0100, 0b0000_0100, 0b0001_1100];

const LONG: Shape = [0b0001_0000, 0b0001_0000, 0b0001_0000, 0b0001_0000];

const BOX: Shape = [0b0000_0000, 0b0000_0000, 0b0001_1000, 0b0001_1000];

const QUEUE: [Shape; 5] = [FLAT, CROSS, CORNER, LONG, BOX];

#[derive(Debug)]
enum Move {
    Left,
    Right,
}

fn collides(a: &u8, b: &u8) -> bool {
    a & b > 0
}

fn fall_piece(
    instrs: &mut dyn Iterator<Item = (usize, &Move)>,
    shapes: &mut dyn Iterator<Item = (usize, &Shape)>,
    mut stack: Vec<u8>,
) -> Vec<u8> {
    let mut shape = *shapes.next().unwrap().1;

    let mut offset = stack.len() + 4;

    loop {
        offset -= 1;
        // do move if doable
        let instr = instrs.next().unwrap().1;

        let can_move = shape
            .iter()
            .rev()
            .zip(offset..)
            .all(|(shape_row, index)| match instr {
                Move::Left => {
                    *shape_row < 64
                        && !collides(&(shape_row << 1), stack.get(index).unwrap_or(&0b0000_0000))
                }
                Move::Right => {
                    (shape_row % 2) == 0
                        && !collides(&(shape_row >> 1), stack.get(index).unwrap_or(&0b0000_0000))
                }
            });
        if can_move {
            shape = shape
                .into_iter()
                .map(|line| match instr {
                    Move::Left => line << 1,
                    Move::Right => line >> 1,
                })
                .collect_vec()
                .try_into()
                .unwrap();
        }

        // check if a row collides if shape were one step lower
        let collision = shape
            .iter()
            .rev()
            .zip(offset - 1..)
            .any(|(shape_row, index)| {
                collides(shape_row, stack.get(index).unwrap_or(&0b0000_0000))
            });

        if collision {
            for (line, index) in shape.iter().rev().zip(offset..) {
                match stack.get_mut(index) {
                    None => {
                        if *line != 0 {
                            stack.push(*line)
                        }
                    }
                    Some(idx) => *idx |= line,
                }
            }
            return stack;
        }
    }
}

#[allow(dead_code)]
fn print_rows(rows: &[u8]) {
    for row in rows.iter().rev() {
        print!("|");
        for i in (0..7).rev() {
            print!("{}", if row >> i & 1 == 0 { ' ' } else { '█' });
        }
        println!("|\n");
    }
}

impl SolutionLinear<Vec<Move>, usize, usize> for Day17Solution {
    fn load(input: &str) -> Result<Vec<Move>> {
        Ok(input
            .chars()
            .map(|c| match c {
                '<' => Move::Left,
                '>' => Move::Right,
                _ => unreachable!(),
            })
            .collect())
    }

    fn part1(input: &mut Vec<Move>) -> Result<usize> {
        let mut instrs = input.iter().enumerate().cycle().peekable();
        let mut shapes = QUEUE.iter().enumerate().cycle();
        let mut stack = vec![0b1111_1111];

        for _ in 0..2022 {
            stack = fall_piece(&mut instrs, &mut shapes, stack);
        }

        Ok(stack.len() - 1) // -1 for 0th row
    }

    fn part2(input: &mut Vec<Move>, _part_1_solution: usize) -> Result<usize> {
        let mut instrs = input.iter().enumerate().cycle().peekable();
        let mut shapes = QUEUE.iter().enumerate().cycle().peekable();
        let mut stack = vec![0b1111_1111];
        let mut recent_history = VecDeque::new();
        let mut seen = HashMap::new();

        // at each step, record the 'fingerprint' of the state (the instruction, shape and top line state) along with how many steps into a cycle we are and the height.
        // when the fingerprint is repeated, find the cycle length and height of the repeated section, and simulate skipping the repeated section by calculating the total
        // height and cycle count.
        // get the remainder manually.
        for step in 1usize.. {
            if recent_history.len() > 10 {
                recent_history.pop_back();
                recent_history.push_front(stack.iter().copied().last().unwrap())
            }

            if seen.contains_key(&(
                instrs.peek().unwrap().0,
                shapes.peek().unwrap().0,
                recent_history.clone(),
            )) {
                let (pre_cycle_step, pre_cycle_height) = seen
                    .get(&(
                        instrs.peek().unwrap().0,
                        shapes.peek().unwrap().0,
                        recent_history,
                    ))
                    .unwrap();

                let cycle_length = step - pre_cycle_step;
                let cycle_height = stack.len() - pre_cycle_height;

                let repeats = 1_000_000_000_000 / cycle_length;
                let height = cycle_height * repeats;

                for _ in 0..(1_000_000_000_000 % cycle_length) {
                    stack = fall_piece(&mut instrs, &mut shapes, stack);
                }

                return Ok(height + stack.len() - pre_cycle_height - cycle_height - 1);
            }

            seen.insert(
                (
                    instrs.peek().unwrap().0,
                    shapes.peek().unwrap().0,
                    recent_history.clone(),
                ),
                (step, stack.len()),
            );
            stack = fall_piece(&mut instrs, &mut shapes, stack);
        }
        unreachable!()
    }
}

pub fn day17(input: &str) -> Result<f32> {
    solve_linear::<Day17Solution, _, _, _>(input)
}

#[cfg(test)]
mod tests {
    use super::Day17Solution;
    use crate::utils::solver_types::SolutionLinear;

    #[test]
    fn test_answer() {
        let input = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

        let p1_expected = 3068;
        let p2_expected = 1514285714288;

        let mut loaded = Day17Solution::load(input).unwrap();
        let p1 = Day17Solution::part1(&mut loaded).unwrap();
        assert_eq!(p1_expected, p1);

        let p2 = Day17Solution::part2(&mut loaded, p1).unwrap();

        assert_eq!(p2_expected, p2);
    }
}
