use crate::Base;
use std::fmt::Display;

pub struct Day01 {
    input: Vec<isize>,
}

impl Day01 {
    pub fn new() -> Day01 {
        return Day01 { input: Vec::new() };
    }
}

impl Base for Day01 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input
            .lines()
            .map(|s| {
                let (dir, len) = s.split_at(1);
                len.parse::<isize>().unwrap() * if dir == "R" { 1 } else { -1 }
            })
            .collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut count = 0;
        let mut dial = 50;
        for r in &self.input {
            dial += r;
            if dial % 100 == 0 {
                count += 1;
            }
        }

        return Box::new(count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut count = 0;
        let mut dial: isize = 50;
        for r in &self.input {
            let start_pos = dial;

            count += (r / 100).abs();

            dial += r % 100;

            if start_pos != 0 && (dial < 0 || dial.abs() > 100) {
                count += 1;
            }

            dial = dial.rem_euclid(100);

            if dial == 0 {
                count += 1;
            }
        }

        return Box::new(count);
    }
}
