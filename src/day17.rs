use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day17)]
pub fn generate(inp: &str) -> Vec<usize> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

fn get_combinations(nums: &[usize], target: usize) -> Vec<Vec<usize>> {
    (1..nums.len())
        .filter_map(|len| {
            let matching = nums
                .iter()
                .copied()
                .combinations(len)
                .filter(|comb| comb.iter().sum::<usize>() == target)
                .collect_vec();
            if matching.is_empty() {
                None
            } else {
                Some(matching)
            }
        })
        .flatten()
        .collect()
}

#[aoc(day17, part1)]
pub fn part1(nums: &[usize]) -> usize {
    get_combinations(nums, 150).len()
}

fn count_min_containers(combs: &mut [Vec<usize>]) -> Option<usize> {
    combs.sort_by_key(Vec::len);

    let len = combs.first().map(Vec::len)?;
    combs.iter().counts_by(Vec::len).get(&len).copied()
}

#[aoc(day17, part2)]
pub fn part2(nums: &[usize]) -> Option<usize> {
    let mut combs = get_combinations(nums, 150);
    count_min_containers(&mut combs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_p1() {
        let inp = vec![20, 15, 10, 5, 5];
        let res = get_combinations(&inp, 25).len();

        assert_eq!(res, 4);
    }

    #[test]
    fn test_sample_input_p2() {
        let inp = vec![20, 15, 10, 5, 5];
        let mut combs = get_combinations(&inp, 25);
        let res = count_min_containers(&mut combs);

        assert!(res.is_some());
        assert_eq!(res.unwrap(), 3);
    }
}
