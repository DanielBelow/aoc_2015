use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day11)]
pub fn generate(inp: &str) -> String {
    String::from(inp)
}

fn has_increasing_straight(password: &str) -> bool {
    password
        .as_bytes()
        .iter()
        .tuple_windows()
        .any(|(a, b, c)| *a + 1 == *b && *b + 1 == *c)
}

fn has_pair_after_index(to_skip: usize, l: char, r: char, text: &str) -> bool {
    text.chars()
        .skip(to_skip)
        .tuple_windows()
        .any(|(ll, rr)| ll == rr && ll != l && rr != r)
}

fn contains_repeating_pair(text: &str) -> bool {
    text.chars()
        .tuple_windows()
        .enumerate()
        .any(|(idx, (l, r))| l == r && has_pair_after_index(idx + 2, l, r, text))
}

fn is_valid_password(password: &str) -> bool {
    if password
        .chars()
        .any(|it| it == 'i' || it == 'o' || it == 'l')
    {
        return false;
    }

    if !has_increasing_straight(password) {
        return false;
    }

    contains_repeating_pair(password)
}

fn increment_password(password: &str) -> String {
    let new_password = password.to_string();

    let mut carry = false;
    let mut first = true;

    new_password
        .chars()
        .rev()
        .fold(String::new(), |mut acc, it| {
            if !first && !carry {
                acc.insert(0, it);
                return acc;
            }

            let new_char = if it < 'z' {
                // invalid chars to skip: 'i', 'o', 'l'
                let inc_by = if it == 'h' || it == 'n' || it == 'k' {
                    2
                } else {
                    1
                };

                carry = false;
                (it as u8 + inc_by) as char
            } else {
                carry = true;
                'a'
            };

            first = false;

            acc.insert(0, new_char);
            acc
        })
}

fn generate_new_password(password: &str) -> String {
    let mut new_password = password.to_string();

    new_password = increment_password(&new_password);
    while !is_valid_password(&new_password) {
        new_password = increment_password(&new_password);
    }

    new_password
}

#[aoc(day11, part1)]
pub fn part1(password: &str) -> String {
    generate_new_password(password)
}

#[aoc(day11, part2)]
pub fn part2(_: &str) -> String {
    let new_password = String::from("hepxxyzz");
    generate_new_password(&new_password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increasing_straight() {
        let inp = "abcasdasd";
        assert!(has_increasing_straight(inp));

        let inp = "cba";
        assert!(!has_increasing_straight(inp));
    }

    #[test]
    fn test_is_valid_password() {
        assert!(is_valid_password("abcdffaa"));
        assert!(is_valid_password("ghjaabcc"));
    }

    #[test]
    fn test_next_password() {
        let test_data = vec![("abcdefgh", "abcdffaa"), ("ghijklmn", "ghjaabcc")];

        for (inp, expected) in test_data {
            let next_pw = generate_new_password(inp);
            assert_eq!(next_pw, expected);
        }
    }
}
