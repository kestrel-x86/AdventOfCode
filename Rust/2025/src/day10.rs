use shared::utils::enumerate_bits;

use crate::Base;
use std::{collections::HashMap, fmt::Display, usize};

#[derive(Clone)]
struct Machine {
    pub lights: u16,
    pub buttons: Vec<u16>,
    pub joltage: Vec<i16>,
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
                    buttons.push({
                        let mut b = 0;
                        for n in segment
                            .trim_matches(|c| c == '(' || c == ')')
                            .split(',')
                            .map(|x| x.parse::<usize>().unwrap())
                        {
                            b |= 1 << n;
                        }
                        b
                    });
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
                joltage,
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

    /*
    This is based off a few ideas from other solutions I saw after I gave up
    trying to figure it out all on my own. I think this is the only puzzle this
    year to have a long explanation. There will be plenty of comments below
    because this one was tough.

    Consider the first machine in the sample input:
    [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

    The easiest way to trim the search is to find a GCD of the joltage numbers
    and divide them down. As far as I can tell, all of the input lines have
    a prime number in the joltage requirements. But we can use specic button
    presses to get to a joltage with a GCD. Specifically, one where all the
    numbers are even.

    To do this, each joltage has be be reduced by either an odd or even number.
    For example, {3,4,5,7} can be reduced by {1,0,1,1} to make them all even.
    Or {3,2,3,1}. As long as the remaining joltage is even, the search area
    can be cut in half. Since all we care about is even or odd numbers, this
    can be represented as a bit mask. Like {1,1,0,1.} Or [##.#]

    Thanks to part 1, we already know how to reach [##.#] in the fewest presses.
    In this machine it is [(3), (0,1)]. One press each gets us joltage {1,1,0,1}.

    If we press these buttons our joltage requirement drops to {2,4,4,6}.

    Now these, being all even numbers, have a gcd of at least 2 (though actually
    2 for this example). If we divide the requirement by 2 we get {1,2,2,3}.

    We can take that {1,2,2,3} and represent even and odd as [#..#]. We go back up
    a few steps and repeat the process, finding the cheapest way to get to [#..#]
    This can repeat via recursion until the requirement is zero, but every time we
    divide the joltage requirement by 2, the subsequent button presses must be
    doubled.

    Once the joltage requirement is {0,0,0,0} the counts can be totaled. If the
    joltage requirement is something like {1,0,0,0} or [#...] it is impossible to
    reach the goal. This means we must drop back a recursion level and try the
    next cheapest button press.

    */

    fn part2(&mut self) -> Box<dyn Display> {
        let mut total = 0;
        for machine in &self.machines {
            total += get_min_presses(
                &machine.joltage.clone(),
                &machine.buttons,
                &mut HashMap::new(),
            );
        }
        return Box::new(total);
    }
}

fn get_min_presses(
    joltage: &Vec<i16>,
    buttons: &Vec<u16>,
    cache: &mut HashMap<Vec<i16>, usize>,
) -> usize {
    if joltage.iter().all(|x| *x == 0) {
        return 0;
    }

    match cache.get(joltage) {
        Some(r) => return *r,
        None => {}
    }

    let mut res = usize::MAX;

    // Odd / even pattern as bit mask
    let pattern = joltage
        .iter()
        .enumerate()
        .map(|(i, j)| if j % 2 == 0 { 0 } else { 1 << i })
        .sum::<u16>();

    // Vec of u16 bitmasks indicating indexes of buttons to press
    for pressed_buttons in find_button_presses_gen(buttons, pattern) {
        let mut next_joltage = joltage.clone();

        for btn_i in enumerate_bits(pressed_buttons as usize) {
            for jltg_i in enumerate_bits(buttons[btn_i] as usize) {
                next_joltage[jltg_i as usize] -= 1;
            }
        }

        // Joltage can't be negative so this button sequence is invalid
        if next_joltage.iter().any(|x| *x < 0) {
            continue;
        }
        next_joltage.iter_mut().for_each(|x| *x /= 2);

        let next_joltage_presses = get_min_presses(&next_joltage, buttons, cache);

        if next_joltage_presses == usize::MAX {
            continue;
        }

        let presses = pressed_buttons.count_ones() as usize + (2 * next_joltage_presses);

        res = res.min(presses);
        if res == 1 {
            break;
        }
    }
    cache.insert(joltage.clone(), res);
    return res;
}

/// Generates bitmasks of button indexes that are able to produce the provided pattern
fn find_button_presses_gen(
    buttons: &Vec<u16>,
    pattern: u16,
) -> impl Iterator<Item = u16> + use<'_> {
    let all_buttons = (1 << buttons.len()) - 1;

    let mut presses = 0u32;
    let max_presses = buttons.len() as u32;

    let mut mask_gen = shared::utils::bit_permutations(0, all_buttons);

    return std::iter::from_fn(move || loop {
        let mask;
        if let Some(m) = mask_gen.next() {
            mask = m;
        } else {
            presses += 1;
            if presses > max_presses {
                return None;
            } else {
                mask_gen = shared::utils::bit_permutations(presses, all_buttons);
                mask = mask_gen.next().unwrap();
            }
        }

        let lights = press(mask as u64, buttons);
        if lights == pattern {
            return Some(mask as u16);
        }
    });
}

// Get bitmask of lights that are on after pressing the buttons indicated by mask
fn press(mask: u64, buttons: &Vec<u16>) -> u16 {
    let mut lights = 0;
    for i in enumerate_bits(mask as usize) {
        lights ^= buttons[i]
    }
    lights
}
