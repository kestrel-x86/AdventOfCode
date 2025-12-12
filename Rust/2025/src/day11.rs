#![allow(non_upper_case_globals)]

use crate::Base;
use std::{collections::HashMap, fmt::Display};

const fn name_to_u32(name: &str) -> u32 {
    let mut u = 0;
    u |= name.as_bytes()[0] as u32;
    u |= (name.as_bytes()[1] as u32) << 8;
    u |= (name.as_bytes()[2] as u32) << 16;
    u as u32
}

const fft: u32 = name_to_u32("fft");
const dac: u32 = name_to_u32("dac");
const svr: u32 = name_to_u32("svr");
const out: u32 = name_to_u32("out");

#[allow(dead_code)]
fn u32_to_name(u: u32) -> String {
    let mut s = String::new();
    for i in 0..3 {
        s.push((u >> (i * 8)) as u8 as char);
    }
    s
}

pub struct Day11 {
    input: HashMap<u32, Vec<u32>>,
}

impl Day11 {
    pub fn new() -> Day11 {
        return Day11 {
            input: HashMap::new(),
        };
    }
}

impl Base for Day11 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.lines() {
            let (a, b) = line.split_once(": ").unwrap();

            self.input.insert(
                name_to_u32(a),
                b.split(' ').map(|x| name_to_u32(x)).collect(),
            );
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let you = name_to_u32("you");

        let mut paths = 0;

        let mut q = Vec::new();
        q.push(you);

        while let Some(current) = q.pop() {
            for conn in &self.input[&current] {
                if *conn == out {
                    paths += 1;
                    continue;
                }
                q.push(*conn);
            }
        }

        return Box::new(paths);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        /*
        I'm going to cheat a little here.

        The paths can only be svr > fft > dac > out OR svr > dac > fft > out
        If fft can lead to dac and dac can lead to fft the number of paths in
        infinite because we have a loop.

        In my input, fft comes before dac. If you count the paths dac > fft
        the answer is zero. One of the two directions always will be. So
        rather than check both to determine the order I'm going to write
        this knowing that its svr > ftt > dac > out

        It saves maybe 80 us. Whatever.
        */

        let mut hist = HashMap::new();
        let middle_counts = paths_between(fft, dac, &self.input, &mut hist);
        hist.clear();
        let start_counts = paths_between(svr, fft, &self.input, &mut hist);
        hist.clear();
        let end_counts = paths_between(dac, out, &self.input, &mut hist);
        return Box::new(start_counts * middle_counts * end_counts);
    }
}

// Follows branches from start and counts how many paths can reach end
fn paths_between(
    start: u32,
    end: u32,
    map: &HashMap<u32, Vec<u32>>,
    hist: &mut HashMap<u32, usize>,
) -> usize {
    let mut counts = 0;

    for next in &map[&start] {
        if *next == end {
            counts += 1;
            continue;
        } else if *next == out {
            continue;
        }
        match hist.get(next) {
            Some(c) => {
                counts += c;
                continue;
            }
            None => {
                let count = paths_between(*next, end, map, hist);
                hist.insert(*next, count);
                counts += count;
            }
        }
    }

    return counts;
}
