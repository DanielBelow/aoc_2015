use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

#[aoc_generator(day3)]
pub fn generate(inp: &str) -> Vec<char> {
    inp.chars().collect()
}

fn deliver_presents(directions: &[char], visited_houses: &mut HashSet<(i64, i64)>) {
    let mut x = 0;
    let mut y = 0;

    for it in directions {
        match *it {
            '^' => y += 1,
            'v' => y -= 1,
            '<' => x -= 1,
            '>' => x += 1,
            _ => panic!("Invalid direction"),
        };

        visited_houses.insert((x, y));
    }
}

#[aoc(day3, part1)]
pub fn part1(v: &[char]) -> usize {
    let mut visited_houses = HashSet::new();
    visited_houses.insert((0, 0));

    deliver_presents(v, &mut visited_houses);
    visited_houses.len()
}

#[aoc(day3, part2)]
pub fn part2(v: &[char]) -> usize {
    let santa_path = v.iter().step_by(2).copied().collect_vec();
    let robo_path = v.iter().skip(1).step_by(2).copied().collect_vec();

    let mut visited_houses = HashSet::new();
    visited_houses.insert((0, 0));

    deliver_presents(&santa_path, &mut visited_houses);
    deliver_presents(&robo_path, &mut visited_houses);
    visited_houses.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_p1() {
        let test_data = vec![(">", 2), ("^>v<", 4), ("^v^v^v^v^v", 2)];

        for (inp, expected) in test_data {
            let data = generate(inp);
            let res = part1(&data);
            assert_eq!(res, expected);
        }
    }

    #[test]
    fn test_sample_input_p2() {
        let test_data = vec![("^v", 3), ("^>v<", 3), ("^v^v^v^v^v", 11)];

        for (inp, expected) in test_data {
            let data = generate(inp);
            let res = part2(&data);
            assert_eq!(res, expected);
        }
    }
}
