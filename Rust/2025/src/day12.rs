use crate::Base;
use std::fmt::Display;

pub struct Day12 {
    package_volumes: Vec<usize>,
    trees: Vec<(usize, Vec<usize>)>,
}

impl Day12 {
    pub fn new() -> Day12 {
        return Day12 {
            package_volumes: Vec::new(),
            trees: Vec::new(),
        };
    }
}

impl Base for Day12 {
    fn parse_input(&mut self, raw_input: String) {
        let mut lines = raw_input.lines();
        while let Some(mut line) = lines.next() {
            if line.as_bytes()[1] == b':' {
                let mut volume = 0;
                line = lines.next().unwrap();

                while !line.is_empty() {
                    volume += line.as_bytes().iter().filter(|b| **b == b'#').count();
                    line = lines.next().unwrap();
                }
                self.package_volumes.push(volume);
            } else {
                let (sz, pkgs) = line.split_once(": ").unwrap();
                let (x, y) = sz.split_once('x').unwrap();
                let tree = (
                    (x.parse::<usize>().unwrap() * y.parse::<usize>().unwrap()),
                    pkgs.split(' ')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect(),
                );

                self.trees.push(tree);
            }
        }
    }

    /*
    This is based on assumptions that seem to be accurate:

    1 Given enough space, the 9*9 packages can be arranged in a shape that tesselates
    2 Any tree that has enough space to contain the volume of required packages can
        contain enough nested packages to pass the check

     */
    fn part1(&mut self) -> Box<dyn Display> {
        let mut count = 0;
        for tree in &self.trees {
            let pkg_vol: usize = tree
                .1
                .iter()
                .enumerate()
                .map(|(i, p)| p * self.package_volumes[i])
                .sum();
            if pkg_vol < tree.0 {
                count += 1;
            }
        }

        return Box::new(count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new("-");
    }
}
