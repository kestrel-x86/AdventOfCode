use shared::bitset::Bitset2D;

use crate::Base;
use std::{collections::HashSet, fmt::Display};

const PAPER: u8 = b'@';

const NEIGHBORS: [(isize, isize); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

pub struct Day04 {
    input: Bitset2D,
}

impl Day04 {
    pub fn new() -> Day04 {
        return Day04 {
            input: Bitset2D::new(0, 0),
        };
    }
}

impl Base for Day04 {
    fn parse_input(&mut self, raw_input: String) {
        let sz = (raw_input.len() as f64).sqrt() as usize;
        let mut map = Bitset2D::new(sz, sz);
        for (y, line) in raw_input.lines().enumerate() {
            for (x, b) in line.as_bytes().iter().enumerate() {
                if *b == PAPER {
                    map.set(x, y);
                }
            }
        }
        self.input = map;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let sz = self.input.rows() as isize;

        let mut count = 0;
        for (x, y) in self.input.iter_set() {
            let mut paper_count = 0;
            for neighbor in NEIGHBORS {
                let nx = x as isize + neighbor.0;
                let ny = y as isize + neighbor.1;
                if nx >= 0
                    && ny >= 0
                    && nx < sz
                    && ny < sz
                    && self.input.is_set(nx as usize, ny as usize)
                {
                    paper_count += 1;
                }
            }
            if paper_count < 4 {
                count += 1;
            }
        }

        return Box::new(count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let sz = self.input.rows() as isize;
        let mut removable = HashSet::new();
        let mut total = 0;

        loop {
            for (x, y) in self.input.iter_set() {
                let mut paper_count = 0;
                for neighbor in NEIGHBORS {
                    let nx = x as isize + neighbor.0;
                    let ny = y as isize + neighbor.1;
                    if nx >= 0
                        && ny >= 0
                        && nx < sz
                        && ny < sz
                        && self.input.is_set(nx as usize, ny as usize)
                    {
                        paper_count += 1;
                    }
                }
                if paper_count < 4 && removable.insert((x, y)) {
                    total += 1;
                }
            }
            if removable.len() == 0 {
                break;
            }

            for (x, y) in removable.drain() {
                self.input.unset(x, y);
            }
        }

        return Box::new(total);
    }
}
