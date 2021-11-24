use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};

const LIGHT_ON: char = '#';
const LIGHT_OFF: char = '.';

#[aoc_generator(day18)]
pub fn generate(inp: &str) -> Vec<Vec<char>> {
    inp.lines().map(|it| it.chars().collect_vec()).collect()
}

fn count_neighbours(grid: &[Vec<char>], row: usize, col: usize, grid_size: usize) -> usize {
    let mut count = 0;

    for (dx, dy) in iproduct!(-1..=1, -1..=1) {
        if dx == 0 && dy == 0 {
            continue;
        }

        let x = usize::try_from(row as i64 + dx).ok();
        let y = usize::try_from(col as i64 + dy).ok();

        match (x, y) {
            (Some(x), Some(y)) if x < grid_size && y < grid_size => {
                let neighbour = grid[x][y];
                count += if neighbour == LIGHT_ON { 1 } else { 0 };
            }
            _ => {}
        };
    }

    count
}

fn single_step(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    let grid_size = grid.len();

    for (row, grid_row) in grid.iter().enumerate() {
        let mut new_row = Vec::new();

        for (col, &cur) in grid_row.iter().enumerate() {
            let num_neighbours = count_neighbours(grid, row, col, grid_size);

            match cur {
                LIGHT_ON if ![2, 3].contains(&num_neighbours) => new_row.push(LIGHT_OFF),
                LIGHT_OFF if num_neighbours == 3 => new_row.push(LIGHT_ON),
                _ => new_row.push(cur),
            };
        }

        assert_eq!(new_row.len(), grid_size);

        result.push(new_row);
    }

    result
}

fn turn_on_corner_lights(grid: &mut Vec<Vec<char>>, grid_size: usize) {
    grid[0][0] = LIGHT_ON;
    grid[0][grid_size] = LIGHT_ON;
    grid[grid_size][0] = LIGHT_ON;
    grid[grid_size][grid_size] = LIGHT_ON;
}

fn run_steps(grid: &[Vec<char>], steps: usize, corners_always_on: bool) -> Vec<Vec<char>> {
    let mut grid = grid.to_vec();

    let grid_size = grid.len() - 1;

    for _ in 0..steps {
        if corners_always_on {
            turn_on_corner_lights(&mut grid, grid_size);
        }

        grid = single_step(&grid);
    }

    if corners_always_on {
        turn_on_corner_lights(&mut grid, grid_size);
    }

    grid
}

fn count_active_lights(grid: &[Vec<char>]) -> usize {
    grid.iter().flatten().filter(|&&it| it == LIGHT_ON).count()
}

#[aoc(day18, part1)]
pub fn part1(grid: &[Vec<char>]) -> usize {
    let transformed = run_steps(grid, 100, false);
    count_active_lights(&transformed)
}

#[aoc(day18, part2)]
pub fn part2(grid: &[Vec<char>]) -> usize {
    let transformed = run_steps(grid, 100, true);
    count_active_lights(&transformed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_p1() {
        let inp = ".#.#.#\n\
                          ...##.\n\
                          #....#\n\
                          ..#...\n\
                          #.#..#\n\
                          ####..";
        let expected = "......\n\
                               ......\n\
                               ..##..\n\
                               ..##..\n\
                               ......\n\
                               ......";
        let expected = generate(expected);

        let gen = generate(inp);
        let res = run_steps(&gen, 4, false);
        assert_eq!(res, expected);

        assert_eq!(count_active_lights(&res), 4);
    }

    #[test]
    fn test_sample_input_p2() {
        let inp = "##.#.#\n\
                          ...##.\n\
                          #....#\n\
                          ..#...\n\
                          #.#..#\n\
                          ####.#";
        let expected = "##.###\n\
                               .##..#\n\
                               .##...\n\
                               .##...\n\
                               #.#...\n\
                               ##...#";
        let expected = generate(expected);

        let gen = generate(inp);
        let res = run_steps(&gen, 5, true);
        assert_eq!(res, expected);

        assert_eq!(count_active_lights(&res), 17);
    }
}
