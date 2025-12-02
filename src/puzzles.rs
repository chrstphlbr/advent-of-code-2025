mod day_01;

use std::collections::HashMap;

use day_01::Day1;

pub trait Puzzle {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn solve(&self) -> &str;
}

pub fn puzzles() -> HashMap<u8, Box<dyn Puzzle>> {
    let mut m: HashMap<u8, Box<dyn Puzzle>> = HashMap::new();

    m.insert(1, Box::new(Day1::new("day_01", "Day 1: Secret Entrance")));

    m
}
