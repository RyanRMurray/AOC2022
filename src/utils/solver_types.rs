use anyhow::Result;
use std::{fmt::Debug, time::Instant};

////////////// SOLUTION LINEAR
/// When a day has two parts that must be solved sequentially
pub trait SolutionLinear<I, S1: Debug, S2: Debug> {
    fn load(&self, input: &str) -> Result<I>;
    fn part1(&self, input: &I) -> Result<S1>;
    fn part2(&self, input: &I, part_1_solution: S1) -> Result<S2>;
}

/// Solve a day where part 2 depends on the output of part 1.
/// Returns the total time elapsed in milliseconds
pub fn solve_linear<I, S1: Debug, S2: Debug>(
    input: &str,
    solution: &impl SolutionLinear<I, S1, S2>,
) -> Result<u128> {
    let start = Instant::now();

    let input = solution.load(input)?;

    let input_loaded = start.elapsed().as_millis();

    println!("Parsed input in:\t{}ms", input_loaded);

    let p1_start = Instant::now();

    let p1 = solution.part1(&input)?;

    let p1_end = p1_start.elapsed().as_millis();

    println!("Part 1 Solution: \t{:?}", p1);
    println!("Part 1 solved in:\t{}ms", p1_end);

    let p2_start = Instant::now();

    let p2 = solution.part2(&input, p1)?;

    let p2_end = p2_start.elapsed().as_millis();

    println!("Part 2 Solution: \t{:?}", p2);
    println!("Part 2 solved in:\t{}ms", p2_end);

    let solved_in = input_loaded + p1_end + p2_end;

    println!("Overall time:\t{}ms", solved_in);

    Ok(solved_in)
}

////////////// SOLUTION SIMULTANEOUS
/// When a day has two parts that can be solved simultaneously
pub trait SolutionSimultaneous<I, S1: Debug, S2: Debug> {
    fn load(&self, input: &str) -> Result<I>;
    fn solve(&self, input: I) -> Result<(S1, S2)>;
}

/// Solve a day where part 1 and part 2 can be solved simultaneously
/// Returns the total time elapsed in milliseconds
pub fn solve_simultaneous<I, S1: Debug, S2: Debug>(
    input: &str,
    solution: &impl SolutionSimultaneous<I, S1, S2>,
) -> Result<u128> {
    let start = Instant::now();

    let input = solution.load(input)?;

    println!("Parsed input in:\t{}ms", start.elapsed().as_millis());

    let loaded = Instant::now();

    let (p1, p2) = solution.solve(input)?;

    println!("Part 1 Solution: \t{:?}", p1);
    println!("Part 2 Solution: \t{:?}", p2);

    println!("Solved in:\t{}ms", loaded.elapsed().as_millis());

    let solved_in = start.elapsed().as_millis();

    println!("Overall time:\t{}ms", solved_in);

    Ok(solved_in)
}
