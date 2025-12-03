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
        for line_result in reader.lines() {
            let line = line_result?;
            let (dir, offset) = dir_offset(&line)?;

            dial.rotate(dir, offset);
        }

        Ok(dial.zeros.to_string())
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
    zeros: u32,
}

impl Dial {
    fn new() -> Self {
        Self {
            max: 99,
            position: 50,
            zeros: 0,
        }
    }

    fn rotate(&mut self, dir: Dir, offset: u32) {
        let mut current_position = i64::from(self.position);

        let mut count = 0;
        while count < offset {
            count += 1;

            let new_position = match dir {
                Dir::Left => current_position - 1,
                Dir::Right => current_position + 1,
            };

            if new_position == -1 {
                current_position = i64::from(self.max);
            } else if new_position == i64::from(self.max) + 1 {
                current_position = 0;
            } else {
                current_position = new_position;
            }

            if current_position == 0 {
                self.zeros += 1
            }
        }

        self.position = current_position.try_into().unwrap(); // should never panic as the loop ensures a positive current_position
    }
}
