use aoc_runner_derive::{aoc, aoc_generator};
use aoc_util::iterator_ext::IteratorExt;
use itertools::iproduct;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug, Copy, Clone)]
#[display("{x},{y}")]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Display, FromStr, Debug, Copy, Clone)]
pub enum Instruction {
    #[display("turn on {0} through {1}")]
    TurnOn(Point, Point),

    #[display("turn off {0} through {1}")]
    TurnOff(Point, Point),

    #[display("toggle {0} through {1}")]
    Toggle(Point, Point),
}

struct Grid<T>
where
    T: Default + Clone,
{
    lights: Vec<T>,
    size: usize,
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    fn with_size(size: usize) -> Self {
        Self {
            lights: vec![T::default(); size * size],
            size,
        }
    }

    fn handle_instruction(&mut self, from: Point, to: Point, f: fn(&mut T)) {
        for (x, y) in iproduct!(from.x..=to.x, from.y..=to.y) {
            let index = x * self.size + y;
            let elem = &mut self.lights[index];
            f(elem);
        }
    }
}

#[aoc_generator(day6)]
pub fn generate(inp: &str) -> Vec<Instruction> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

#[aoc(day6, part1)]
pub fn part1(insts: &[Instruction]) -> usize {
    let mut g = Grid::with_size(1_000);

    for instr in insts {
        match *instr {
            Instruction::TurnOn(from, to) => g.handle_instruction(from, to, |it| *it = true),
            Instruction::TurnOff(from, to) => g.handle_instruction(from, to, |it| *it = false),
            Instruction::Toggle(from, to) => g.handle_instruction(from, to, |it| *it = !*it),
        };
    }

    g.lights.iter().count_if(|it| **it)
}

#[aoc(day6, part2)]
pub fn part2(insts: &[Instruction]) -> usize {
    let mut g = Grid::<usize>::with_size(1_000);

    for instr in insts {
        match *instr {
            Instruction::TurnOn(from, to) => g.handle_instruction(from, to, |it| *it += 1),
            Instruction::TurnOff(from, to) => {
                g.handle_instruction(from, to, |it| *it = it.saturating_sub(1));
            }
            Instruction::Toggle(from, to) => g.handle_instruction(from, to, |it| *it += 2),
        }
    }

    g.lights.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_p1() {
        let test_data = vec![
            ("turn on 0,0 through 999,999", 1000 * 1000),
            ("toggle 0,0 through 999,0", 1000),
        ];

        for (inp, expected) in test_data {
            let data = generate(inp);
            let res = part1(&data);
            assert_eq!(res, expected);
        }
    }

    #[test]
    fn test_sample_input_p2() {
        let test_data = vec![
            ("turn on 0,0 through 0,0", 1),
            ("toggle 0,0 through 999,999", 2_000_000),
        ];

        for (inp, expected) in test_data {
            let data = generate(inp);
            let res = part2(&data);
            assert_eq!(res, expected);
        }
    }
}
