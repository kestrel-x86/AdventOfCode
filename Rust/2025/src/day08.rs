use crate::Base;
use std::fmt::Display;

pub struct Day08 {
    input: String,
}

impl Day08 {
    pub fn new() -> Day08 {
        return Day08 { input: String::new() };
    }
}

impl Base for Day08 {
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
