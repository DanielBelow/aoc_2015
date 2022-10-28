use aoc_runner_derive::{aoc, aoc_generator};
use aoc_util::iterator_ext::IteratorExt;
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::HashSet;

#[derive(Display, FromStr, Debug, Clone)]
#[display("{from} => {to}")]
struct Production {
    from: String,
    to: String,
}

#[derive(Debug)]
pub struct InputData {
    productions: Vec<Production>,
    molecule: String,
}

#[aoc_generator(day19)]
pub fn generate(inp: &str) -> InputData {
    let productions = inp
        .lines()
        .dropping_back(1)
        .filter_map(|it| it.parse::<Production>().ok())
        .collect_vec();

    let molecule = inp.lines().last().map_or(String::new(), String::from);

    InputData {
        productions,
        molecule,
    }
}

#[aoc(day19, part1)]
pub fn part1(data: &InputData) -> usize {
    let mut result: HashSet<String> = HashSet::new();

    let molecule = &data.molecule;
    for repl in &data.productions {
        let from = &repl.from;
        let to = &repl.to;

        data.molecule.match_indices(from).for_each(|(idx, it)| {
            let mut new_mol = molecule.clone();
            new_mol.replace_range(idx..idx + it.len(), to);
            result.insert(new_mol);
        });
    }

    result.len()
}

fn split_molecules(molecule: &str) -> Vec<String> {
    let mut result = Vec::new();

    for (a, b) in molecule
        .chars()
        .tuple_windows()
        .filter(|(a, _)| a.is_uppercase())
    {
        let mut a_str = a.to_string();
        if !b.is_uppercase() {
            a_str.push(b);
        }

        result.push(a_str);
    }

    if let Some(ch) = molecule.chars().last() {
        if ch.is_uppercase() {
            result.push(ch.to_string());
        }
    }

    result
}

fn count_steps(molecule: &str) -> usize {
    let molecules = split_molecules(molecule);
    let total = molecules.len();
    let num_rn_ar = molecules.iter().count_if(|it| it.eq(&"Rn") || it.eq(&"Ar"));
    let num_y = molecules.iter().count_if(|it| it.eq(&"Y"));

    total - num_rn_ar - 2 * num_y - 1
}

#[aoc(day19, part2)]
pub fn part2(data: &InputData) -> usize {
    count_steps(&data.molecule)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let inp = "H => HO\n\
                          H => OH\n\
                          O => HH\n\
                          HOH";

        let gen = generate(inp);
        let res = part1(&gen);
        assert_eq!(res, 4);
    }
}
