use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::HashMap;

#[derive(Display, FromStr, Clone, Debug)]
#[display("{name} can fly {speed} km/s for {flight_duration} seconds, but then must rest for {rest_duration} seconds.")]
pub struct Reindeer {
    name: String,
    speed: u64,
    flight_duration: u64,
    rest_duration: u64,
}

#[aoc_generator(day14)]
pub fn generate(inp: &str) -> Vec<Reindeer> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

fn travel_distance(reindeer: &Reindeer, mut travel_time: u64) -> u64 {
    let mut distance = 0;

    while travel_time != 0 {
        let time_traveled = reindeer.flight_duration.min(travel_time);
        distance += reindeer.speed * time_traveled;

        // Time spent flying
        travel_time = travel_time.saturating_sub(time_traveled);

        // Time spent resting
        travel_time = travel_time.saturating_sub(reindeer.rest_duration);
    }

    distance
}

fn race_for(data: &[Reindeer], seconds: u64) -> u64 {
    data.iter()
        .map(|it| travel_distance(it, seconds))
        .max()
        .unwrap_or_default()
}

fn leading_reindeer_after(data: &[Reindeer], seconds: u64) -> Vec<String> {
    let positions = data
        .iter()
        .map(|it| (it, travel_distance(it, seconds)))
        .sorted_by_key(|(_, it)| *it)
        .rev()
        .collect_vec();

    let (_, dist) = positions.iter().max_by_key(|(_, dist)| *dist).unwrap();

    positions
        .iter()
        .take_while(|(_, d)| *d == *dist)
        .map(|(it, _)| it.name.clone())
        .collect()
}

#[aoc(day14, part1)]
pub fn part1(data: &[Reindeer]) -> u64 {
    race_for(data, 2_503)
}

#[aoc(day14, part2)]
pub fn part2(data: &[Reindeer]) -> Option<i64> {
    let mut reindeer_points = HashMap::new();
    for t in 1..=2_503 {
        let leading_reindeer = leading_reindeer_after(data, t);
        for r in leading_reindeer {
            reindeer_points
                .entry(r)
                .and_modify(|it| *it += 1)
                .or_insert(1);
        }
    }

    reindeer_points.values().copied().max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_p1() {
        let test_data = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\n\
                                Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";

        let gen = generate(test_data);
        let res = race_for(&gen, 1_000);
        assert_eq!(res, 1120);
    }
}
