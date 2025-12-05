use crate::Base;
use std::fmt::Display;

pub struct Day03 {
    input: Vec<Vec<u8>>,
}

impl Day03 {
    pub fn new() -> Day03 {
        return Day03 { input: Vec::new() };
    }
}

impl Base for Day03 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.lines().map(|l| l.bytes().collect()).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut total = 0usize;
        for bank in &self.input {
            let tens = bank[..(bank.len() - 1)].iter().max().unwrap();
            let first_max = bank.iter().position(|x| x == tens).unwrap();
            let ones = bank[(first_max + 1)..].iter().max().unwrap();
            let tens = tens - 48;
            let ones = ones - 48;
            total += ((tens * 10) + ones) as usize;
        }

        return Box::new(total);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut total = 0;
        for bank in &self.input {
            let mut bank: &[u8] = bank;
            for place in (0..12).rev() {
                let num = bank[..(bank.len() - place)].iter().max().unwrap();
                let num_i = bank.iter().position(|x| x == num).unwrap();
                bank = &bank[(num_i + 1)..];
                total += (*num - 48) as usize * 10usize.pow(place as u32);
            }
        }
        return Box::new(total); // 172664333119298
    }
}
