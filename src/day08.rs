use aoc_runner_derive::{aoc, aoc_generator};
use aoc_util::iterator_ext::IteratorExt;
use lazy_static::lazy_static;
use regex::Regex;

#[aoc_generator(day8)]
pub fn generate(inp: &str) -> Vec<String> {
    inp.lines().map(String::from).collect()
}

fn string_value(s: &str) -> usize {
    lazy_static! {
        static ref RE: Regex = Regex::new("(\\\\x[0-9A-Fa-f]{2})").unwrap();
    }

    let mut s = s.replace("\\\\", "x");
    s = s.replace("\\\"", "x");
    s = RE.replace_all(&s, "x").to_string();

    s.len() - 2
}

#[aoc(day8, part1)]
pub fn part1(lines: &[String]) -> usize {
    lines.iter().sum_by(|it| it.len() - string_value(it))
}

#[aoc(day8, part2)]
pub fn part2(lines: &[String]) -> usize {
    lines
        .iter()
        .map(|it| it.escape_default().count() + 2 - it.len())
        .sum()
}
