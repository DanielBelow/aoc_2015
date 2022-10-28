use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day24)]
pub fn generate(inp: &str) -> Vec<usize> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

fn find_sum(weights: &[usize], num_groups: usize) -> Option<usize> {
    let total_weight = weights.iter().sum::<usize>();
    let weight_per_group = total_weight / num_groups;

    for len in 1..weights.len() {
        let min_weight = weights
            .iter()
            .combinations(len)
            .filter(|it| it.iter().copied().sum::<usize>() == weight_per_group)
            .map(|it| it.iter().copied().product::<usize>())
            .min();

        if min_weight.is_some() {
            return min_weight;
        }
    }

    None
}

#[aoc(day24, part1)]
pub fn part1(weights: &[usize]) -> Option<usize> {
    find_sum(weights, 3)
}

#[aoc(day24, part2)]
pub fn part2(weights: &[usize]) -> Option<usize> {
    find_sum(weights, 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let gen = vec![1usize, 2, 3, 4, 5, 7, 8, 9, 10, 11];

        let res = part1(&gen);
        assert!(res.is_some());
        assert_eq!(res.unwrap(), 99);
    }
}
