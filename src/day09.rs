use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::collections::HashSet;

#[derive(Display, FromStr, Debug, Clone)]
#[display("{lhs} to {rhs} = {dist}")]
pub struct Connection {
    lhs: String,
    rhs: String,
    dist: usize,
}

impl Connection {
    fn connects(&self, city_a: &str, city_b: &str) -> bool {
        (self.lhs.eq(city_a) && self.rhs.eq(city_b)) || (self.lhs.eq(city_b) && self.rhs.eq(city_a))
    }
}

#[aoc_generator(day9)]
pub fn generate(inp: &str) -> Vec<Connection> {
    inp.lines().filter_map(|it| it.parse().ok()).collect()
}

fn get_nodes(conns: &[Connection]) -> HashSet<&String> {
    let mut nodes = HashSet::new();

    for it in conns {
        nodes.insert(&it.lhs);
        nodes.insert(&it.rhs);
    }

    nodes
}

fn find_connection<'a>(from: &str, to: &str, conns: &'a [Connection]) -> Option<&'a Connection> {
    conns.iter().find(|it| it.connects(from, to))
}

fn route_length(route: &[&String], conns: &[Connection]) -> usize {
    let mut result = 0;

    for window in route.windows(2) {
        if let [from, to] = &window {
            if let Some(conn) = find_connection(from, to, conns) {
                result += conn.dist;
            }
        }
    }

    result
}

fn get_distances(conns: &[Connection]) -> Vec<usize> {
    let nodes = get_nodes(conns);
    let num_nodes = nodes.len();

    nodes
        .into_iter()
        .permutations(num_nodes)
        .map(|it| route_length(&it, conns))
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(conns: &[Connection]) -> Option<usize> {
    get_distances(conns).into_iter().min()
}

#[aoc(day9, part2)]
pub fn part2(conns: &[Connection]) -> Option<usize> {
    get_distances(conns).into_iter().max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        let inp = "London to Dublin = 464\n\
            London to Belfast = 518\n\
            Dublin to Belfast = 141";

        let gen = generate(inp);
        let res = part1(&gen);
        assert_eq!(res, Some(605));
    }

    #[test]
    fn test_sample_p2() {
        let inp = "London to Dublin = 464\n\
            London to Belfast = 518\n\
            Dublin to Belfast = 141";

        let gen = generate(inp);
        let res = part2(&gen);
        assert!(res.is_some());
        assert_eq!(res, Some(982));
    }
}
