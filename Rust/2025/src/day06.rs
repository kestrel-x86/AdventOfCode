use crate::Base;
use std::fmt::Display;

pub struct Day06 {
    input: String,
}

impl Day06 {
    pub fn new() -> Day06 {
        return Day06 { input: String::new() };
    }
}

impl Base for Day06 {
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
