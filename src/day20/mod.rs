use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day20)]
pub fn generate(inp: &str) -> u64 {
    inp.parse().unwrap_or_default()
}

fn find_first_house(target: u64, presents_per_elf: u64, stop_after: Option<usize>) -> u64 {
    const LIMIT: u32 = 1_000_000;

    let mut house_map = HashMap::new();

    for elf in 1u32..=LIMIT {
        let num_presents = u64::from(elf) * presents_per_elf;

        let houses = if let Some(stop_after) = stop_after {
            (elf..).step_by(elf as usize).take(stop_after).collect_vec()
        } else {
            (elf..)
                .step_by(elf as usize)
                .take_while(|&it| it < LIMIT)
                .collect_vec()
        };

        for house in houses {
            house_map
                .entry(u64::from(house))
                .and_modify(|it| *it += num_presents)
                .or_insert_with(|| num_presents);
        }
    }

    house_map
        .iter()
        .filter(|(_, v)| **v >= target)
        .min_by_key(|(k, _)| **k)
        .map(|(k, _)| *k)
        .unwrap_or_default()
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[aoc(day20, part1)]
pub fn part1(inp: &u64) -> u64 {
    find_first_house(*inp, 10, None)
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[aoc(day20, part2)]
pub fn part2(inp: &u64) -> u64 {
    find_first_house(*inp, 11, Some(50))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        assert_eq!(find_first_house(10, 10, None), 1);
        assert_eq!(find_first_house(30, 10, None), 2);
        assert_eq!(find_first_house(40, 10, None), 3);
        assert_eq!(find_first_house(70, 10, None), 4);
    }
}
