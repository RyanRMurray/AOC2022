use std::{fmt::Debug, time::Instant};

////////////// SOLUTION LINEAR
/// When a day has two parts that must be solved sequentially
pub trait SolutionLinear<I1, I2, S1: Debug, S2: Debug> {
    fn get_part_1_input(&self) -> &I1;
    fn get_part_1_solution(&self) -> &S1;
    fn get_part_2_input(&self) -> &I2;
    fn get_part_2_solution(&self) -> &S2;

    fn load(&mut self, input: &str);
    fn part1(&mut self);
    fn part2(&mut self);
}

pub fn solve_linear<I1, I2, S1: Debug, S2: Debug>(
    input: &str,
    solution: &mut impl SolutionLinear<I1, I2, S1, S2>,
) -> u128 {
    let start = Instant::now();

    solution.load(input);

    println!("Parsed input in:\t{}ms", start.elapsed().as_millis());

    let loaded = Instant::now();

    solution.part1();

    println!("Part 1 Solution: \t{:?}", solution.get_part_1_solution());
    println!("Part 1 solved in:\t{}ms", loaded.elapsed().as_millis());

    let part_1_solved = Instant::now();

    solution.part2();

    println!("Part 2 Solution: \t{:?}", solution.get_part_2_solution());
    println!(
        "Part 2 solved in:\t{}ms",
        part_1_solved.elapsed().as_millis()
    );

    let solved_in = start.elapsed().as_millis();

    println!("Overall time:\t{}ms", solved_in);

    solved_in
}

////////////// SOLUTION SIMULTANEOUS
/// When a day has two parts that can be solved simultaneously
pub trait SolutionSimultaneous<I, S1: Debug, S2: Debug> {
    fn get_input(&self) -> &I;
    fn get_part_1_solution(&self) -> &S1;
    fn get_part_2_solution(&self) -> &S2;

    fn load(&mut self, input: &str);
    fn solve(&mut self);
}

pub fn solve_simultaneous<I, S1: Debug, S2: Debug>(
    input: &str,
    solution: &mut impl SolutionSimultaneous<I, S1, S2>,
) -> u128 {
    let start = Instant::now();

    solution.load(input);

    println!("Parsed input in:\t{}ms", start.elapsed().as_millis());

    let loaded = Instant::now();

    solution.solve();

    println!("Part 1 Solution: \t{:?}", solution.get_part_1_solution());
    println!("Part 2 Solution: \t{:?}", solution.get_part_2_solution());

    println!("Solved in:\t{}ms", loaded.elapsed().as_millis());

    let solved_in = start.elapsed().as_millis();

    println!("Overall time:\t{}ms", solved_in);

    solved_in
}
