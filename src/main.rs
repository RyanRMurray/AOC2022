use clap::Parser;

use crate::{
    solutions::templates::{
        linear_template::ExampleSolutionLinear, simultaneous_template::ExampleSolutionSimultaneous,
    },
    utils::solver_types::{solve_linear, solve_simultaneous},
};

pub mod solutions;
pub mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, short)]
    example: bool,
}

fn main() {
    let args = Args::parse();

    if args.example {
        run_example();
        return;
    }
    println!("Hello, world!");
}

fn run_example() {
    println!("Here's an example of a linear solution:");
    print!(
        r"
input: [1,2,3,4,5]
part 1: sum up these numbers
part 2: multiply the result of part 1 by the number of numbers in the input
"
    );

    let solved_in_1 = solve_linear("[1,2,3,4,5]", &ExampleSolutionLinear::default()).unwrap();

    println!("\nHere's an example of a simultaneous solution:");
    print!(
        r"
input: [6,5,4,2,3,5,8]
part 1: get the first number that's higher than the previous
part 2: get the number after the first number that's higher than the previous
"
    );

    let solved_in_2 =
        solve_simultaneous("[6,5,4,2,3,5,8]", &ExampleSolutionSimultaneous::default()).unwrap();

    println!("Overall time: {}ms", solved_in_1 + solved_in_2);
}
