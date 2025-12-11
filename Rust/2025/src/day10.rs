use crate::Base;
use std::fmt::Display;

struct Machine {
    pub lights: u16,
    pub buttons: Vec<Vec<u16>>,
    pub _joltage: Vec<u16>,
}

pub struct Day10 {
    machines: Vec<Machine>,
}

impl Day10 {
    pub fn new() -> Day10 {
        return Day10 {
            machines: Vec::new(),
        };
    }
}

impl Base for Day10 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.lines() {
            let mut lights = 0;
            let mut buttons = Vec::new();
            let mut joltage = Vec::new();
            for segment in line.split(' ') {
                if segment.starts_with('[') {
                    for (i, l) in segment
                        .trim_matches(|c| c == '[' || c == ']')
                        .as_bytes()
                        .iter()
                        .enumerate()
                    {
                        if *l == b'#' {
                            lights |= 1 << i;
                        }
                    }
                } else if segment.starts_with('(') {
                    buttons.push(
                        segment
                            .trim_matches(|c| c == '(' || c == ')')
                            .split(',')
                            .map(|x| x.parse().unwrap())
                            .collect(),
                    );
                } else if segment.starts_with('{') {
                    joltage = segment
                        .trim_matches(|c| c == '{' || c == '}')
                        .split(',')
                        .map(|x| x.parse().unwrap())
                        .collect();
                }
            }
            self.machines.push(Machine {
                lights: lights,
                buttons: buttons,
                _joltage: joltage,
            });
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut total_presses = 0;
        for m in &self.machines {
            let all_buttons = (1 << m.buttons.len()) - 1;
            'o: for presses in 1..=m.buttons.len() {
                for mask in shared::utils::bit_permutations(presses as u32, all_buttons) {
                    let lights = press(mask as u64, &m.buttons);
                    if lights == m.lights {
                        total_presses += presses;
                        break 'o;
                    }
                }
            }
        }
        return Box::new(total_presses);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new("-");
    }
}

// Get lights that are on after pressing the buttons indicated by mask
fn press(mask: u64, buttons: &Vec<Vec<u16>>) -> u16 {
    let mut lights = 0;
    for i in 0..buttons.len() {
        if mask & (1 << i) != 0 {
            for btn in &buttons[i] {
                lights ^= 1 << btn;
            }
        }
    }
    lights
}
