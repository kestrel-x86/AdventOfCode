use crate::Base;
use std::fmt::Display;

pub struct Day06 {
    input: String,
}

impl Day06 {
    pub fn new() -> Day06 {
        return Day06 {
            input: String::new(),
        };
    }
}

impl Base for Day06 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let lines: Vec<&str> = self.input.lines().collect();
        let mut input: Vec<Vec<usize>> = Vec::new();
        let symbols: Vec<char>;

        {
            let len = lines.len();
            for i in 0..(len - 1) {
                input.push(
                    lines[i]
                        .split_whitespace()
                        .map(|x| x.parse().unwrap())
                        .collect(),
                );
            }

            symbols = lines[len - 1]
                .split_whitespace()
                .map(|x| x.chars().nth(0).unwrap())
                .collect();
        }

        let mut grand_total = 0;
        let len = input.len();

        for (i, symbol) in symbols.iter().enumerate() {
            if *symbol == '*' {
                let mut prod = 1;
                for line in 0..len {
                    prod *= input[line][i];
                }
                grand_total += prod;
            } else {
                let mut sum = 0;
                for line in 0..len {
                    sum += input[line][i];
                }
                grand_total += sum;
            }
        }

        return Box::new(grand_total);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let lines: Vec<&str> = self.input.lines().collect();

        let number_lines: Vec<Vec<char>>;
        let symbols: Vec<char>;
        {
            let len = lines.len();
            number_lines = (0..(len - 1))
                .map(|i| {
                    let nl: Vec<char> = lines[i].chars().collect();
                    // This block pads lines to the right with ' ' to make them all equal length
                    // My puzzle input does not require this, but the sample data does
                    // let max_w = lines.iter().map(|x| x.len()).max().unwrap();
                    // let d = max_w - lines[i].len();
                    // nl.append(&mut (0..d).map(|_| ' ').collect::<Vec<char>>());
                    nl
                })
                .collect();

            symbols = lines[len - 1]
                .chars()
                .filter_map(|x| if x == '*' || x == '+' { Some(x) } else { None })
                .collect();
        }

        let mut grand_total = 0;
        let mut symbol_i = 0;

        let mut sum_or_prod = move |nums: &mut Vec<usize>| -> usize {
            let r = match symbols[symbol_i] {
                '*' => nums.drain(..).product::<usize>(),
                '+' => nums.drain(..).sum::<usize>(),
                _ => unreachable!(),
            };
            symbol_i += 1;
            r
        };

        let mut nums: Vec<usize> = Vec::new();
        for current_col in 0..number_lines[0].len() {
            let n = number_lines
                .iter()
                .map(|line| line[current_col])
                .collect::<String>();
            let n = n.trim();

            if n.len() == 0 {
                grand_total += sum_or_prod(&mut nums);
                continue;
            }

            nums.push(n.parse().unwrap());
        }

        grand_total += sum_or_prod(&mut nums);

        return Box::new(grand_total);
    }
}
