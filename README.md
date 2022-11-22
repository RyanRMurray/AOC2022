# AOC2022

Advent of Code is an event run by Eric Wastl. For each day leading up to Christmas, programmers are tasked with solving two small problems each day.
[Find more information here.](https://adventofcode.com/2022/about)

## Using this project

You can run this project by calling

```bash
cargo build
./target/debug/aoc2022.exe [day number] [input file path]
```

or

```bash
cargo run [day number] [input file path]
```

By default, `./inputs/input_{day}.txt` will be used as the input file path. For example, day 01 will use `./inputs/input_01.txt`

## Contribution

Before contributing, run the following:
```
pip install pre-commit
pre-commit install
```
This will run a number of linting actions and tests before you can commit.