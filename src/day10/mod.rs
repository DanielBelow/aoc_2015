use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day10)]
pub fn generate(inp: &str) -> usize {
    inp.parse().unwrap()
}

fn get_next_num(n: &str) -> String {
    let mut result = String::new();

    let mut iter = n.chars();

    while let Some(cur) = &iter.next() {
        let num_chars = 1 + iter.peeking_take_while(|it| *it == *cur).count();
        result.push_str(&num_chars.to_string());
        result.push(*cur);
    }

    result
}

fn play_n_rounds(n: usize, start_num: &str) -> usize {
    let mut cur = start_num.to_string();
    for _ in 0..n {
        cur = get_next_num(&cur);
    }

    cur.len()
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[aoc(day10, part1)]
pub fn part1(num: &usize) -> usize {
    let num = num.to_string();
    play_n_rounds(40, &num)
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[aoc(day10, part2)]
pub fn part2(num: &usize) -> usize {
    let num = num.to_string();
    play_n_rounds(50, &num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_number_gen() {
        let test_data = vec![
            ("1", "11"),
            ("11", "21"),
            ("21", "1211"),
            ("1211", "111221"),
            ("111221", "312211"),
        ];

        for (inp, expected) in test_data {
            let data = generate(inp);
            let next_num = get_next_num(&data.to_string());
            assert_eq!(next_num, expected);
        }
    }
}
