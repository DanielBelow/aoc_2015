use aoc_runner_derive::{aoc, aoc_generator};
use aoc_util::iterator_ext::IteratorExt;
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Debug)]
#[display("{length}x{width}x{height}")]
pub struct BoxDimensions {
    length: usize,
    width: usize,
    height: usize,
}

impl BoxDimensions {
    fn side_a(&self) -> usize {
        self.length * self.width
    }

    fn side_b(&self) -> usize {
        self.width * self.height
    }

    fn side_c(&self) -> usize {
        self.height * self.length
    }

    pub fn dimensions(&self) -> Vec<usize> {
        let mut dims = vec![self.length, self.width, self.height];
        dims.sort_unstable();
        dims
    }

    pub fn volume(&self) -> usize {
        self.length * self.width * self.height
    }

    pub fn surface_area(&self) -> usize {
        (2 * self.side_a()) + (2 * self.side_b()) + (2 * self.side_c())
    }

    pub fn smallest_side(&self) -> usize {
        self.side_a().min(self.side_b().min(self.side_c()))
    }
}

#[aoc_generator(day2)]
pub fn generate(inp: &str) -> Vec<BoxDimensions> {
    inp.lines()
        .filter_map(|it| it.parse().map_err(|e| panic!("{e:?}")).ok())
        .collect()
}

fn calculate_paper(box_dim: &BoxDimensions) -> usize {
    box_dim.surface_area() + box_dim.smallest_side()
}

fn ribbon_length(box_dim: &BoxDimensions) -> Option<usize> {
    let dims = box_dim.dimensions();
    match dims[..] {
        [first, second, ..] => Some(2 * first + 2 * second + box_dim.volume()),
        _ => None,
    }
}

#[aoc(day2, part1)]
pub fn part1(v: &[BoxDimensions]) -> usize {
    v.iter().sum_by(calculate_paper)
}

#[aoc(day2, part2)]
pub fn part2(v: &[BoxDimensions]) -> usize {
    v.iter().filter_map(ribbon_length).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_p1() {
        let test_data = vec![("2x3x4", 58), ("1x1x10", 43)];

        for (inp, expected) in test_data {
            let data = generate(inp);
            let res = part1(&data);
            assert_eq!(res, expected);
        }
    }

    #[test]
    fn test_sample_input_p2() {
        let test_data = vec![("2x3x4", 34), ("1x1x10", 14)];

        for (inp, expected) in test_data {
            let data = generate(inp);
            let res = part2(&data);
            assert_eq!(res, expected);
        }
    }
}
