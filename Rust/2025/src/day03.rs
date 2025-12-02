use crate::Base;
use std::fmt::Display;

pub struct Day03 {
    input: String,
}

impl Day03 {
    pub fn new() -> Day03 {
        return Day03 { input: String::new() };
    }
}

impl Base for Day03 {
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
