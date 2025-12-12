use shared::v3i::Vector3Int as V3;

use crate::Base;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fmt::Display,
};

pub struct Day08 {
    input: Vec<V3>,
}

impl Day08 {
    pub fn new() -> Day08 {
        return Day08 { input: Vec::new() };
    }
}

impl Base for Day08 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input
            .lines()
            .map(|line| {
                let mut n = line.split(",");
                V3::new(
                    n.next().unwrap().parse().unwrap(),
                    n.next().unwrap().parse().unwrap(),
                    n.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut groups: Vec<HashSet<usize>> = (0..self.input.len())
            .map(|i| {
                let mut hs = HashSet::new();
                hs.insert(i);
                hs
            })
            .collect();

        let mut heap = BinaryHeap::new();
        for i in 0..(self.input.len() - 1) {
            for j in (i + 1)..self.input.len() {
                let d = self.input[i].dist_sq(&self.input[j]);
                heap.push(Reverse((d, i, j)));
            }
        }

        for _ in 0..1000 {
            do_one(&mut heap, &mut groups);
        }

        groups.sort_by(|a, b| b.len().cmp(&a.len()));

        let total = groups.iter().take(3).map(|gp| gp.len()).product::<usize>();

        return Box::new(total);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut groups: Vec<HashSet<usize>> = (0..self.input.len())
            .map(|i| {
                let mut hs = HashSet::new();
                hs.insert(i);
                hs
            })
            .collect();

        let mut heap = BinaryHeap::new();
        for i in 0..(self.input.len() - 1) {
            for j in (i + 1)..self.input.len() {
                let d = self.input[i].dist_sq(&self.input[j]);
                heap.push(Reverse((d, i, j)));
            }
        }

        loop {
            let (a, b) = do_one(&mut heap, &mut groups);

            if groups[0].len() == self.input.len() {
                return Box::new(self.input[a].x * self.input[b].x);
            }
        }
    }
}

fn do_one(
    heap: &mut BinaryHeap<Reverse<(usize, usize, usize)>>,
    groups: &mut Vec<HashSet<usize>>,
) -> (usize, usize) {
    let Reverse((_, a, b)) = heap.pop().unwrap();
    let group_a = groups.iter().position(|hs| hs.contains(&a)).unwrap();
    let group_b = groups.iter().position(|hs| hs.contains(&b)).unwrap();
    if group_a == group_b {
        return (a, b);
    }

    groups[group_a] = groups[group_a]
        .union(&groups[group_b])
        .map(|x| *x)
        .collect();

    groups.remove(group_b);
    (a, b)
}
