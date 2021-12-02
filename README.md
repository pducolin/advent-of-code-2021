# Advent of Code 2021
Solutions to Advent of Code 2021

I'm using this project to play with Rust, TDD (Test Driven Development) and pre-commit hooks
- [Advent of Code 2021](#advent-of-code-2021)
  - [Testing](#testing)
    - [Continuous testing](#continuous-testing)
  - [Github Actions](#github-actions)
  - [TIL](#til)
    - [Resources](#resources)
    - [Rust learnings](#rust-learnings)
      - [Day 01](#day-01)
      - [Day 02](#day-02)

## Testing

### Continuous testing

## Github Actions

Github built-in CI/CD is free for public repositories since Aug, 2019. It has many workflow templates, including one for Python applications. To add it and start running linting and tests on Github, click on Actions -> New Workflow -> Python Applications. This will create a new configuration `yaml` under `.github/workflows`, that by defaults execute the actions at every push on `main` branch 

## TIL

Here's a collection of resources and learnings from 2021 edition

### Resources

- [Rust by example](https://github.com/rust-lang/rust-by-example) a collection of examples to learn Rust
- [AoC 2020 in Rust](https://github.com/duarten/advent-of-code/tree/main/aoc2020) inspired my project structure
- [Rust formatter](https://github.com/rust-lang/rustfmt)

### Rust learnings

#### Day 01

Installed Rust on my laptop following [official website instructions](https://www.rust-lang.org/tools/install)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Using [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension in VS Code, I preferred its developer experience compared to the offical extension.

Rust often uses a `Maybe` like monad, `Result`. Its value can be accessed using `unwrap`

```rust
let file = File::open("inputs/day01.txt").unwrap();
```

Variables are immutable by default, aka constants: their value cannot be changed, unless they are explicitly declared mutable using `mut`

```rust
let mut counter = 0;
```

I use `cargo run --bin dayXX` to build and execute my daily binary

```bash
cargo run --bin day01
```

#### Day 02

Structs in Rust

```rust
struct Instruction {
  direction: String,
  count: i32,
}

let instruction = {direction: 'down', count: 5};
```

Extract an array of `Instruction` struct from a file:

```rust
let instructions: Vec<Instruction> = io::BufReader::new(file)
            .lines()
            .map(|line|line.unwrap())
            .map(|line|{
              let splits: Vec<&str> = line.split_whitespace().collect();
              Instruction{direction: String::from(splits[0]), count: splits[1].parse().unwrap()}
            })
            .collect();
```

