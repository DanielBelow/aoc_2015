use aoc_runner_derive::aoc;

fn generate_md5(secret_key: &str, number: u64) -> String {
    let text = format!("{secret_key}{number}");

    let digest = md5::compute(text);
    format!("{digest:x}")
}

#[aoc(day4, part1)]
pub fn part1(v: &str) -> u64 {
    for num in 0u64.. {
        let hash = generate_md5(v, num);
        if hash.starts_with("00000") {
            return num;
        }
    }

    unreachable!("No u64 solution for given input...");
}

#[aoc(day4, part2)]
pub fn part2(v: &str) -> u64 {
    for num in 0u64.. {
        let hash = generate_md5(v, num);
        if hash.starts_with("000000") {
            return num;
        }
    }

    unreachable!("No u64 solution for given input...");
}
