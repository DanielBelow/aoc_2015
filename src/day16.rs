use aoc_runner_derive::{aoc, aoc_generator};
use regex::{Error, Regex};
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
pub struct Sue {
    idx: usize,
    children: Option<usize>,
    cats: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
    goldfish: Option<usize>,
    trees: Option<usize>,
    cars: Option<usize>,
    perfumes: Option<usize>,
}

fn get_regex(s: &str) -> Result<Regex, Error> {
    let pattern = format!("{s}: ([0-9]+)");
    Regex::new(&pattern)
}

fn get_item(s: &str, item: &str) -> Option<usize> {
    let re = get_regex(item).ok()?;
    if re.is_match(s) {
        re.captures(s)
            .and_then(|it| it.get(1))
            .and_then(|it| it.as_str().parse().ok())
    } else {
        None
    }
}

fn get_idx(s: &str) -> Option<usize> {
    let re = Regex::new("Sue ([0-9]+)").ok()?;
    re.captures(s)
        .and_then(|it| it.get(1))
        .and_then(|it| it.as_str().parse().ok())
}

impl FromStr for Sue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(idx) = get_idx(s) else {
            return Err(());
        };

        Ok(Self {
            idx,
            akitas: get_item(s, "akitas"),
            cars: get_item(s, "cars"),
            cats: get_item(s, "cats"),
            children: get_item(s, "children"),
            goldfish: get_item(s, "goldfish"),
            perfumes: get_item(s, "perfumes"),
            pomeranians: get_item(s, "pomeranians"),
            samoyeds: get_item(s, "samoyeds"),
            trees: get_item(s, "trees"),
            vizslas: get_item(s, "vizslas"),
        })
    }
}

#[aoc_generator(day16)]
pub fn generate(inp: &str) -> Vec<Sue> {
    inp.lines()
        .filter_map(|it| it.parse::<Sue>().ok())
        .collect()
}

fn matches_items(sue: &Sue, cat_tree_ord: Ordering, pomeranian_fish_ord: Ordering) -> bool {
    sue.akitas.map_or(true, |a| a == 0)
        && sue.cars.map_or(true, |c| c == 2)
        && sue.cats.map_or(true, |c| c.cmp(&7) == cat_tree_ord)
        && sue.children.map_or(true, |c| c == 3)
        && sue
            .goldfish
            .map_or(true, |g| g.cmp(&5) == pomeranian_fish_ord)
        && sue.perfumes.map_or(true, |p| p == 1)
        && sue
            .pomeranians
            .map_or(true, |p| p.cmp(&3) == pomeranian_fish_ord)
        && sue.samoyeds.map_or(true, |s| s == 2)
        && sue.trees.map_or(true, |t| t.cmp(&3) == cat_tree_ord)
        && sue.vizslas.map_or(true, |v| v == 0)
}

#[aoc(day16, part1)]
pub fn part1(sues: &[Sue]) -> Option<usize> {
    sues.iter().find_map(|it| {
        if matches_items(it, Equal, Equal) {
            Some(it.idx)
        } else {
            None
        }
    })
}

#[aoc(day16, part2)]
pub fn part2(sues: &[Sue]) -> Option<usize> {
    sues.iter().find_map(|it| {
        if matches_items(it, Greater, Less) {
            Some(it.idx)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let inp = "Sue 1: goldfish: 6, trees: 9, akitas: 0";
        let data = generate(inp);
        assert_eq!(data.len(), 1);

        let sue = data.first();

        assert_eq!(
            sue,
            Some(&Sue {
                idx: 1,
                children: None,
                cats: None,
                goldfish: Some(6),
                trees: Some(9),
                cars: None,
                akitas: Some(0),
                pomeranians: None,
                samoyeds: None,
                vizslas: None,
                perfumes: None,
            })
        );
    }
}
