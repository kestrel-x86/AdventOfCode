use std::env;
use std::fmt::Display;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

mod runner;
pub mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        print_usage();
        return;
    }
    let day: usize = args[1].parse::<usize>().unwrap();

    let part = match args.len() {
        2 => runner::Part::Both,
        p if p >= 3 => {
            if args[2].parse::<usize>().unwrap() == 1 {
                runner::Part::One
            } else {
                runner::Part::Two
            }
        }
        _ => {
            print_usage();
            return;
        }
    };

    runner::run_day(day, part);
}

fn print_usage() {
    println!("Usage: \ncargo run [day] <part>");
}

pub trait Base {
    fn parse_input(&mut self, raw_input: String);
    fn part1(&mut self) -> Box<dyn Display>;
    fn part2(&mut self) -> Box<dyn Display>;
}
