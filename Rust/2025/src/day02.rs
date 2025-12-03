use crate::Base;
use std::fmt::Display;

pub struct Day02 {
    input: Vec<(usize, usize)>,
}

impl Day02 {
    pub fn new() -> Day02 {
        return Day02 { input: Vec::new() };
    }
}

impl Base for Day02 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input
            .split(',')
            .map(|r| {
                let (a, b) = r.split_once('-').unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect()
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut sum = 0;
        for range in &self.input {
            let mut current = range.0;
            while current <= range.1 {
                let len = 1 + current.ilog10();
                if len % 2 == 1 {
                    current = 10usize.pow(len);
                    continue;
                }
                let div = 10usize.pow(len / 2);
                let a = current / div;
                let b = current % div;
                if a == b {
                    sum += current;
                }
                current += 1;
            }
        }

        return Box::new(sum);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut sum = 0;
        for range in &self.input {
            let mut current = range.0;
            while current <= range.1 {
                if is_pattern(current) {
                    sum += current;
                }
                current += 1;
            }
        }

        return Box::new(sum);
    }
}

fn is_pattern(num: usize) -> bool {
    let s = num.to_string();
    let bytes = s.as_bytes();

    let len = bytes.len();
    let half_len = len / 2;

    for pat_len in 1..=half_len {
        if len % pat_len != 0 {
            continue;
        }

        let repeats = len / pat_len;
        let pat = &bytes[0..pat_len];
        let mut ok = true;
        for r in 1..repeats {
            if pat != &bytes[(r * pat_len)..((r + 1) * pat_len)] {
                ok = false;
                break;
            }
        }
        if ok {
            return true;
        }
    }
    false
}
