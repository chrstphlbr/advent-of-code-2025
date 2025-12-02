use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(name = "aoc25", author, version, about = "Advent of Code 2025")]
struct Cli {
    day: u8,
}

fn main() -> Result<()> {
    println!("Welcome to Advent of Code 2025!\n");

    let cli = Cli::parse();

    println!("Run puzzle from day {}", cli.day);

    Ok(())
}
