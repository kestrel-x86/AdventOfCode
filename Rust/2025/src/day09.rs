use crate::Base;
use shared::v2i::Vector2Int as V2;
use std::fmt::Display;

pub struct Day09 {
    input: Vec<V2>,
}

impl Day09 {
    pub fn new() -> Day09 {
        return Day09 { input: Vec::new() };
    }
}

impl Base for Day09 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input
            .lines()
            .map(|line| {
                let (a, b) = line.split_once(',').unwrap();
                V2::new(a.parse().unwrap(), b.parse().unwrap())
            })
            .collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut a = 0;
        for i in 0..(self.input.len() - 1) {
            for j in (i + 1)..self.input.len() {
                a = a.max(
                    (self.input[i].x - self.input[j].x + 1).abs()
                        * (self.input[i].y - self.input[j].y + 1).abs(),
                );
            }
        }

        return Box::new(a);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut a = 0;
        for i in 0..(self.input.len() - 2) {
            for j in (i + 2)..self.input.len() {
                let mut rect_y_range = [self.input[i].y, self.input[j].y];
                rect_y_range.sort();
                let mut rect_x_range = [self.input[i].x, self.input[j].x];
                rect_x_range.sort();

                let mut fail = false;
                for k in 0..(self.input.len() - 1) {
                    if self.input[k].x == self.input[k + 1].x {
                        fail = intersects_v(
                            self.input[k].x,
                            self.input[k].y,
                            self.input[k + 1].y,
                            &rect_x_range,
                            &rect_y_range,
                        );
                    } else {
                        fail = intersects_h(
                            self.input[k].y,
                            self.input[k].x,
                            self.input[k + 1].x,
                            &rect_x_range,
                            &rect_y_range,
                        );
                    }
                    if fail {
                        break;
                    }
                }
                if !fail {
                    a = a.max(
                        ((self.input[i].x - self.input[j].x).abs() + 1)
                            * ((self.input[i].y - self.input[j].y).abs() + 1),
                    );
                }
            }
        }
        return Box::new(a);
    }
}

fn intersects_h(
    line_y: isize,
    line_x1: isize,
    line_x2: isize,
    rect_x: &[isize; 2],
    rect_y: &[isize; 2],
) -> bool {
    let line_x_min = line_x1.min(line_x2);
    let line_x_max = line_x1.max(line_x2);
    if !(rect_y[0] < line_y && line_y < rect_y[1]) {
        return false;
    }

    return line_x_max > rect_x[0] && line_x_min < rect_x[1];
}

fn intersects_v(
    line_x: isize,
    line_y1: isize,
    line_y2: isize,
    rect_x: &[isize; 2],
    rect_y: &[isize; 2],
) -> bool {
    let line_y_min = line_y1.min(line_y2);
    let line_y_max = line_y1.max(line_y2);

    if !(rect_x[0] < line_x && line_x < rect_x[1]) {
        return false;
    }

    return line_y_max > rect_y[0] && line_y_min < rect_y[1];
}
