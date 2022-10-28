use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day20)]
pub fn generate(inp: &str) -> u64 {
    inp.parse().unwrap_or_default()
}

fn find_lowest_house_number(house_map: &HashMap<u64, u64>, target: u64) -> Option<u64> {
    house_map
        .iter()
        .filter(|(_, v)| **v >= target)
        .min_by_key(|(k, _)| **k)
        .map(|(k, _)| *k)
}

fn update_house(house_map: &mut HashMap<u64, u64>, house: u32, num_presents: u64) {
    house_map
        .entry(u64::from(house))
        .and_modify(|it| *it += num_presents)
        .or_insert_with(|| num_presents);
}

fn find_first_house_with_limit(
    target: u64,
    presents_per_elf: u64,
    stop_after: usize,
) -> Option<u64> {
    const LIMIT: u32 = 1_000_000;

    let mut house_map = HashMap::new();

    for elf in 1u32..=LIMIT {
        let num_presents = u64::from(elf) * presents_per_elf;

        for house in (elf..).step_by(elf as usize).take(stop_after) {
            update_house(&mut house_map, house, num_presents);
        }
    }

    find_lowest_house_number(&house_map, target)
}

fn find_first_house(target: u64, presents_per_elf: u64) -> Option<u64> {
    const LIMIT: u32 = 1_000_000;

    let mut house_map = HashMap::new();

    for elf in 1u32..=LIMIT {
        let num_presents = u64::from(elf) * presents_per_elf;

        for house in (elf..).step_by(elf as usize).take_while(|&it| it < LIMIT) {
            update_house(&mut house_map, house, num_presents);
        }
    }

    find_lowest_house_number(&house_map, target)
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[aoc(day20, part1)]
pub fn part1(inp: &u64) -> Option<u64> {
    find_first_house(*inp, 10)
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[aoc(day20, part2)]
pub fn part2(inp: &u64) -> Option<u64> {
    find_first_house_with_limit(*inp, 11, 50)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        assert_eq!(find_first_house(10, 10), Some(1));
        assert_eq!(find_first_house(30, 10), Some(2));
        assert_eq!(find_first_house(40, 10), Some(3));
        assert_eq!(find_first_house(70, 10), Some(4));
    }
}
