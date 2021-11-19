use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn generate(inp: &str) -> Vec<i64> {
    inp.chars()
        .map(|it| match it {
            '(' => 1,
            ')' => -1,
            _ => panic!("Invalid character found"),
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(v: &[i64]) -> i64 {
    v.iter().sum()
}

#[aoc(day1, part2)]
pub fn part2(v: &[i64]) -> usize {
    let mut res = 0;

    for (idx, &elem) in v.iter().enumerate() {
        res += elem;
        if res.is_negative() {
            return idx + 1;
        }
    }

    unreachable!("Santa never went into the basement!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let inp = "(())";
        let data = generate(inp);
        assert_eq!(data, vec![1, 1, -1, -1]);
    }

    #[test]
    fn test_sample_input_p1() {
        let test_data = vec![
            ("(())", 0i64),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ];

        for (inp, expected) in test_data {
            let data = generate(inp);
            let res = part1(&data);
            assert_eq!(res, expected);
        }
    }

    #[test]
    fn test_sample_input_p2() {
        let test_data = vec![(")", 1), ("()())", 5)];

        for (inp, expected) in test_data {
            let data = generate(inp);
            let res = part2(&data);
            assert_eq!(res, expected);
        }
    }
}
