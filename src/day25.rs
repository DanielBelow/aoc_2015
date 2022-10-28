use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("To continue, please consult the code grid in the manual.  Enter the code at row {row}, column {col}.")]
pub struct Entry {
    row: usize,
    col: usize,
}

#[aoc_generator(day25)]
pub fn generate(inp: &str) -> Option<Entry> {
    inp.parse().ok()
}

fn sum_first_n_digits(n: usize) -> usize {
    n * (n + 1) / 2
}

fn get_index_in_cantor(row: usize, col: usize) -> usize {
    sum_first_n_digits(col) + sum_first_n_digits(col + row - 2) - sum_first_n_digits(col - 1)
}

#[aoc(day25, part1)]
pub fn part1(inp: &Entry) -> usize {
    let num_repetitions = get_index_in_cantor(inp.row, inp.col);
    (1..num_repetitions).fold(20_151_125, |acc, _| (acc * 252_533) % 33_554_393)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cantor_indices() {
        let indices = [
            (1, 1),
            (2, 1),
            (1, 2),
            (3, 1),
            (2, 2),
            (1, 3),
            (4, 1),
            (3, 2),
            (2, 3),
            (1, 4),
        ];

        for (expected, &(row, col)) in indices.iter().enumerate() {
            let res = get_index_in_cantor(row, col);
            assert_eq!(res, expected + 1);
        }
    }
}
