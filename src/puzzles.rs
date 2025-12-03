mod day_01;
mod day_02;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use anyhow::Result;

use day_01::Day1;
use day_02::Day2;

pub trait Puzzle {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn solve(&self) -> Result<String>;

    fn input_reader(&self) -> Result<BufReader<File>> {
        let path = format!("assets/{}-input.txt", self.id());
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(reader)
    }
}

pub fn puzzles() -> HashMap<u8, Box<dyn Puzzle>> {
    let mut m: HashMap<u8, Box<dyn Puzzle>> = HashMap::new();

    m.insert(1, Box::new(Day1::new("day_01", "Day 1: Secret Entrance")));
    m.insert(2, Box::new(Day2::new("day_02", "Day 2: Gift Shop")));

    m
}
