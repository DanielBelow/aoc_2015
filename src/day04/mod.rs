use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
pub fn generate(inp: &str) -> String {
    inp.to_string()
}

fn generate_md5(secret_key: &str, number: u64) -> String {
    let mut text = secret_key.to_string();
    text.push_str(number.to_string().as_str());

    let digest = md5::compute(text);
    format!("{:x}", digest)
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
