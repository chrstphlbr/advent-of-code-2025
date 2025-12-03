use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Result, anyhow, bail};
use derive_more::Display;

use crate::puzzles::Puzzle;

pub struct Day1 {
    id: String,
    name: String,
}

impl Day1 {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: String::from(id),
            name: String::from(name),
        }
    }
}

impl Puzzle for Day1 {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn solve(&self) -> Result<String> {
        let path = format!("assets/{}-input.txt", self.id());
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut dial = Dial::new();
        let mut count = 0;
        for line_result in reader.lines() {
            let line = line_result?;
            let (dir, offset) = dir_offset(&line)?;

            dial.rotate(dir, offset);

            if dial.position == 0 {
                count += 1;
            }
        }

        Ok(count.to_string())
    }
}

#[derive(Display)]
enum Dir {
    Left,
    Right,
}

fn dir_offset(line: &str) -> Result<(Dir, u32)> {
    if line.len() < 2 {
        bail!("line length ({}) < 2", line.len());
    }

    let dir_char = line
        .chars()
        .nth(0)
        .ok_or_else(|| anyhow!("could not get first char of line"))?;

    let dir = match dir_char {
        'L' => Dir::Left,
        'R' => Dir::Right,
        _ => bail!("invalid direction char '{}'", dir_char),
    };

    let offset = line[1..].parse()?;

    Ok((dir, offset))
}

struct Dial {
    max: u32,
    position: u32,
}

impl Dial {
    fn new() -> Self {
        Self {
            max: 99,
            position: 50,
        }
    }

    fn rotate(&mut self, dir: Dir, offset: u32) {
        let new_position = match dir {
            Dir::Left => self.rotate_left(offset),
            Dir::Right => self.rotate_right(offset),
        };

        self.position = new_position;
    }

    fn rotate_left(&self, offset: u32) -> u32 {
        let max = i64::from(self.max); // never panics as u32 always into i64
        let offset = i64::from(offset); // never panics as u32 always into i64
        let position = i64::from(self.position); // never panics as u32 fits into i64
        let mut new_position: i64 = position - offset;
        while new_position < 0 {
            new_position = new_position + (max + 1) // add + 1 because the Dial starts at 0
        }
        new_position.try_into().unwrap() // should never panic as the loop ensures a positive new_position
    }

    fn rotate_right(&self, offset: u32) -> u32 {
        let mut new_position = self.position + offset;
        while new_position > self.max {
            new_position = new_position - (self.max + 1) // add + 1 because the Dial starts at 0
        }
        new_position
    }
}
