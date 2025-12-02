use crate::Base;
use std::fmt::Display;

pub struct Day04 {
    input: String,
}

impl Day04 {
    pub fn new() -> Day04 {
        return Day04 { input: String::new() };
    }
}

impl Base for Day04 {
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
