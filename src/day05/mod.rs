use aoc_runner_derive::{aoc, aoc_generator};
use aoc_util::iterator_ext::IteratorExt;
use itertools::Itertools;

#[aoc_generator(day5)]
pub fn generate(inp: &str) -> Vec<String> {
    inp.lines().map(String::from).collect()
}

fn contains_vowels(num: usize, text: &str) -> bool {
    text.chars()
        .count_if(|it| matches!(it, 'a' | 'e' | 'i' | 'o' | 'u'))
        >= num
}

fn contains_invalid_pair(invalid_pairs: &[&str], text: &str) -> bool {
    invalid_pairs.iter().any(|it| text.contains(it))
}

fn has_letter_pair(text: &str) -> bool {
    text.chars().tuple_windows().any(|(l, r)| l == r)
}

fn is_nice_string_p1(text: &str) -> bool {
    const INVALID_PAIRS: &[&str] = &["ab", "cd", "pq", "xy"];
    !contains_invalid_pair(INVALID_PAIRS, text) && has_letter_pair(text) && contains_vowels(3, text)
}

fn has_pair_after_index(to_skip: usize, l: char, r: char, text: &str) -> bool {
    text.chars()
        .skip(to_skip)
        .tuple_windows()
        .any(|(ll, rr)| ll == l && rr == r)
}

fn contains_separated_pair(text: &str) -> bool {
    text.chars().tuple_windows().any(|(l, _, r)| l == r)
}

fn contains_repeating_pair(text: &str) -> bool {
    text.chars()
        .tuple_windows()
        .enumerate()
        .any(|(idx, (l, r))| has_pair_after_index(idx + 2, l, r, text))
}

fn is_nice_string_p2(text: &str) -> bool {
    contains_separated_pair(text) && contains_repeating_pair(text)
}

#[aoc(day5, part1)]
pub fn part1(v: &[String]) -> usize {
    v.iter()
        .fold(0, |acc, it| acc + usize::from(is_nice_string_p1(it)))
}

#[aoc(day5, part2)]
pub fn part2(v: &[String]) -> u64 {
    v.iter()
        .fold(0, |acc, it| acc + u64::from(is_nice_string_p2(it)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_p1() {
        let test_data = vec![
            ("ugknbfddgicrmopn", true),
            ("aaa", true),
            ("jchzalrnumimnmhp", false),
            ("haegwjzuvuyypxyu", false),
            ("dvszwmarrgswjxmb", false),
        ];

        for (inp, expected) in test_data {
            assert_eq!(is_nice_string_p1(inp), expected);
        }
    }

    #[test]
    fn test_sample_input_p2() {
        let test_data = vec![
            ("qjhvhtzxzqqjkmpb", true),
            ("xxyxx", true),
            ("uurcxstgmygtbstg", false),
            ("ieodomkazucvgmuy", false),
            ("aaa", false),
        ];

        for (inp, expected) in test_data {
            let asd = is_nice_string_p2(inp);
            assert_eq!(asd, expected);
        }
    }
}
