use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day20)]
pub fn generate(inp: &str) -> u64 {
    inp.parse().unwrap_or_default()
}

fn find_first_house<P>(target: u64, presents_per_elf: u64, pred: P) -> Option<u64>
where
    P: Fn(u64, u64) -> bool,
{
    const LIMIT: u64 = 1_000_000;

    if presents_per_elf >= target {
        return Some(1);
    } else if 2 * presents_per_elf + presents_per_elf >= target {
        return Some(2);
    }

    (3..=LIMIT)
        .map(|elf| {
            let mut divs = divisors::get_divisors(elf);
            divs.push(1);
            divs.push(elf);

            divs.retain(|&it| pred(it, elf));

            let sum = divs.iter().sum::<u64>() * presents_per_elf;
            (elf, sum)
        })
        .find_map(|(elf, sum)| if sum >= target { Some(elf) } else { None })
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[aoc(day20, part1)]
pub fn part1(inp: &u64) -> Option<u64> {
    find_first_house(*inp, 10, |_, _| true)
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[aoc(day20, part2)]
pub fn part2(inp: &u64) -> Option<u64> {
    find_first_house(*inp, 11, |div, elf| div * 50 > elf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        assert_eq!(find_first_house(10, 10, |_, _| true), Some(1));
        assert_eq!(find_first_house(30, 10, |_, _| true), Some(2));
        assert_eq!(find_first_house(40, 10, |_, _| true), Some(3));
        assert_eq!(find_first_house(70, 10, |_, _| true), Some(4));
    }
}
