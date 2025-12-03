use std::io::BufRead;

use anyhow::{Result, anyhow};

use crate::puzzles::Puzzle;

pub struct Day2 {
    id: String,
    name: String,
}

impl Day2 {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: String::from(id),
            name: String::from(name),
        }
    }
}

impl Puzzle for Day2 {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn solve(&self) -> Result<String> {
        let reader = self.input_reader()?;

        let line = reader
            .lines()
            .next()
            .ok_or_else(|| anyhow!("could not read input line"))??;

        todo!("needs implementing")
    }
}

struct Range {
    from: i32,
    to: i32,
}
