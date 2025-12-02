use crate::Base;
use std::fmt::Display;

pub struct Day07 {
    input: String,
}

impl Day07 {
    pub fn new() -> Day07 {
        return Day07 { input: String::new() };
    }
}

impl Base for Day07 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        return Box::new("-");
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new("-");
    }
}
