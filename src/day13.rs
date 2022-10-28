use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::HashMap;

#[derive(Display, FromStr, Debug, Clone)]
enum Connection {
    #[display("{0} would gain {1} happiness units by sitting next to {2}.")]
    Increase(String, i64, String),

    #[display("{0} would lose {1} happiness units by sitting next to {2}.")]
    Decrease(String, i64, String),
}

pub struct PeopleData {
    people: Vec<String>,
    happiness: HashMap<String, HashMap<String, i64>>,
}

#[aoc_generator(day13)]
pub fn generate(inp: &str) -> PeopleData {
    let happiness: HashMap<String, HashMap<String, i64>> = inp
        .lines()
        .filter_map(|it| it.parse::<Connection>().ok())
        .fold(HashMap::new(), |mut acc, it| {
            let (pa, val, pb) = match it {
                Connection::Increase(pa, val, pb) => (pa, val, pb),
                Connection::Decrease(pa, val, pb) => (pa, -val, pb),
            };

            acc.entry(pa)
                .and_modify(|e| {
                    e.insert(pb.clone(), val);
                })
                .or_insert_with(|| HashMap::from([(pb, val)]));

            acc
        });

    PeopleData {
        people: happiness.keys().cloned().collect_vec(),
        happiness,
    }
}

fn happiness_for_seating_pair(
    person_a: &str,
    person_b: &str,
    data: &HashMap<String, HashMap<String, i64>>,
) -> i64 {
    let a_happiness = data
        .get(person_a)
        .and_then(|it| it.get(person_b).copied())
        .unwrap_or_default();

    let b_happiness = data
        .get(person_b)
        .and_then(|it| it.get(person_a).copied())
        .unwrap_or_default();

    a_happiness + b_happiness
}

fn calc_happiness(order: &[&String], data: &HashMap<String, HashMap<String, i64>>) -> i64 {
    let mut result = 0;

    for window in order.windows(2) {
        if let [person_a, person_b] = &window {
            result += happiness_for_seating_pair(person_a, person_b, data);
        }
    }

    // match first and last too
    if let Some(person_a) = order.first() {
        if let Some(person_b) = order.last() {
            result += happiness_for_seating_pair(person_a, person_b, data);
        }
    }

    result
}

fn find_best_seating_arrangement(
    data: &HashMap<String, HashMap<String, i64>>,
    people: &[String],
) -> i64 {
    let num_people = people.len();

    people
        .iter()
        .permutations(num_people)
        .map(|it| calc_happiness(&it, data))
        .max()
        .unwrap_or_default()
}

#[aoc(day13, part1)]
pub fn part1(data: &PeopleData) -> i64 {
    let people = &data.people;
    find_best_seating_arrangement(&data.happiness, people)
}

#[aoc(day13, part2)]
pub fn part2(data: &PeopleData) -> i64 {
    let mut people = data.people.clone();
    people.push(String::from("Self"));

    find_best_seating_arrangement(&data.happiness, &people)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_p1() {
        let test_data = "Alice would gain 54 happiness units by sitting next to Bob.\n\
                                Alice would lose 79 happiness units by sitting next to Carol.\n\
                                Alice would lose 2 happiness units by sitting next to David.\n\
                                Bob would gain 83 happiness units by sitting next to Alice.\n\
                                Bob would lose 7 happiness units by sitting next to Carol.\n\
                                Bob would lose 63 happiness units by sitting next to David.\n\
                                Carol would lose 62 happiness units by sitting next to Alice.\n\
                                Carol would gain 60 happiness units by sitting next to Bob.\n\
                                Carol would gain 55 happiness units by sitting next to David.\n\
                                David would gain 46 happiness units by sitting next to Alice.\n\
                                David would lose 7 happiness units by sitting next to Bob.\n\
                                David would gain 41 happiness units by sitting next to Carol.";

        let gen = generate(test_data);
        let res = part1(&gen);
        assert_eq!(res, 330);
    }
}
