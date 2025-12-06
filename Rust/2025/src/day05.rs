use crate::Base;
use std::{fmt::Display, ops::Range};

pub struct Day05 {
    ranges: Vec<Range<usize>>,
    ingredients: Vec<usize>,
}

impl Day05 {
    pub fn new() -> Day05 {
        return Day05 {
            ranges: Vec::new(),
            ingredients: Vec::new(),
        };
    }
}

impl Base for Day05 {
    fn parse_input(&mut self, raw_input: String) {
        let (a, b) = raw_input.split_once("\n\n").unwrap();
        for line in a.lines() {
            let (s, e) = line.split_once('-').unwrap();
            self.ranges
                .push(s.parse().unwrap()..(e.parse::<usize>().unwrap() + 1));
        }
        for line in b.lines() {
            self.ingredients.push(line.parse().unwrap());
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut total = 0;

        for ing in &self.ingredients {
            for range in &self.ranges {
                if range.contains(&ing) {
                    total += 1;
                    break;
                }
            }
        }

        return Box::new(total);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        reduce_ranges(&mut self.ranges);

        let mut t = 0;
        for r in &self.ranges {
            t += r.end - r.start;
        }
        return Box::new(t);
    }
}

fn reduce_ranges(ranges: &mut Vec<Range<usize>>) {
    ranges.sort_by(|a, b| a.start.cmp(&b.start));

    let mut changes = true;
    let mut len = ranges.len();
    while changes {
        changes = false;
        let mut i = 0;
        while i < (len - 1) {
            let mut j = i + 1;
            while j < len {
                if ranges[i].start <= ranges[j].end && ranges[j].start <= ranges[i].end {
                    ranges[i].start = ranges[i].start.min(ranges[j].start);
                    ranges[i].end = ranges[i].end.max(ranges[j].end);
                    ranges.remove(j);
                    len -= 1;
                    changes = true;
                }
                if ranges[j].start > ranges[i].end {
                    break;
                }
                j += 1;
            }
            i += 1;
        }
        if !changes {
            break;
        }
    }
}
