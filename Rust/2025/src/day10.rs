use crate::Base;
use std::fmt::Display;

pub struct Day10 {
    input: String,
}

impl Day10 {
    pub fn new() -> Day10 {
        return Day10 { input: String::new() };
    }
}

impl Base for Day10 {
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
