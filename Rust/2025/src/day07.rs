use shared::bitset::Bitset2D;

use crate::Base;
use std::{collections::HashSet, fmt::Display};

pub struct Day07 {
    start_x: usize,
    splitter_map: Bitset2D,
    raw_input: String,
}

impl Day07 {
    pub fn new() -> Day07 {
        return Day07 {
            start_x: 0,
            splitter_map: Bitset2D::new(0, 0),
            raw_input: String::new(),
        };
    }
}

impl Base for Day07 {
    fn parse_input(&mut self, raw_input: String) {
        let y = raw_input.lines().count();
        let x = raw_input.lines().next().unwrap().len();

        self.splitter_map = Bitset2D::new(x, y);

        for (y, line) in raw_input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        self.start_x = x;
                    }
                    '^' => {
                        self.splitter_map.set(x, y);
                    }
                    _ => {}
                }
            }
        }
        self.raw_input = raw_input.to_string();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let max_y = self.splitter_map.rows();

        let mut beams = HashSet::new();
        beams.insert((self.start_x, 0));

        let mut splits = 0;

        loop {
            let mut new_beams = HashSet::new();
            for (x, mut y) in beams.drain() {
                y += 1;
                if y > max_y {
                    continue;
                }
                if self.splitter_map.is_set(x, y) {
                    splits += 1;
                    new_beams.insert((x + 1, y));
                    new_beams.insert((x - 1, y));
                } else {
                    new_beams.insert((x, y));
                }
            }

            if new_beams.len() == 0 {
                break;
            }
            beams = new_beams;
        }

        return Box::new(splits);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let max_x = self.raw_input.lines().nth(0).unwrap().len();

        let mut tl_arr: Vec<usize> = (0..=max_x).map(|_| 0).collect();
        let mut new_tl_arr: Vec<usize> = (0..=max_x).map(|_| 0).collect();

        tl_arr[self.start_x] = 1;
        for line in self.raw_input.lines() {
            for (x, val) in tl_arr.iter().enumerate() {
                if *val == 0 {
                    continue;
                }
                if line.as_bytes()[x] == b'^' {
                    new_tl_arr[x + 1] += val;
                    new_tl_arr[x - 1] += val;
                } else {
                    new_tl_arr[x] += val;
                }
            }
            std::mem::swap(&mut tl_arr, &mut new_tl_arr);
            new_tl_arr.fill(0);
        }

        return Box::new(tl_arr.iter().sum::<usize>());
    }
}
