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

    fn solve(&self) -> &str {
        todo!()
    }
}
