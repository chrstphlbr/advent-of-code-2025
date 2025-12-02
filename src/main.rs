mod puzzles;

use anyhow::{Result, anyhow};
use clap::Parser;

use puzzles::puzzles;

#[derive(Parser)]
#[command(name = "aoc25", author, version, about = "Advent of Code 2025")]

struct Cli {
    day: u8,
}

fn main() -> Result<()> {
    println!("Welcome to Advent of Code 2025!\n");

    let cli = Cli::parse();

    let puzzles = puzzles();
    let puzzle = puzzles
        .get(&cli.day)
        .ok_or_else(|| anyhow!("Day {} does not exist", cli.day))?;

    println!("Solve puzzle \"{}\":", puzzle.name());

    let result = puzzle.solve()?;

    println!("{}", result);

    Ok(())
}
