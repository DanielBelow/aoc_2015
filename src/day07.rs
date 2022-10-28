use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};
use std::collections::HashMap;
use std::ops::{BitAnd, BitOr, Shl, Shr};

#[derive(Display, FromStr, Debug, Clone)]
pub enum Connection {
    #[display("{0} -> {1}")]
    ValueInput(u16, String),

    #[display("{0} AND {1} -> {2}")]
    AndValue(u16, String, String),

    #[display("{0} AND {1} -> {2}")]
    And(String, String, String),

    #[display("{0} OR {1} -> {2}")]
    Or(String, String, String),

    #[display("{0} LSHIFT {1} -> {2}")]
    LeftShift(String, u16, String),

    #[display("{0} RSHIFT {1} -> {2}")]
    RightShift(String, u16, String),

    #[display("NOT {0} -> {1}")]
    Not(String, String),

    #[display("{0} -> {1}")]
    SignalInput(String, String),
}

#[aoc_generator(day7)]
pub fn generate(inp: &str) -> HashMap<String, Connection> {
    inp.lines()
        .filter_map(|it| it.parse::<Connection>().ok())
        .fold(HashMap::new(), |mut acc, it| {
            let wire = match &it {
                Connection::ValueInput(_, out)
                | Connection::AndValue(_, _, out)
                | Connection::And(_, _, out)
                | Connection::Or(_, _, out)
                | Connection::LeftShift(_, _, out)
                | Connection::RightShift(_, _, out)
                | Connection::Not(_, out)
                | Connection::SignalInput(_, out) => out,
            };

            acc.insert(wire.clone(), it);
            acc
        })
}

fn get_signal_on(
    conns: &HashMap<String, Connection>,
    signal: &str,
    cache: &mut HashMap<String, u16>,
) -> u16 {
    if cache.contains_key(signal) {
        return *cache.get(signal).unwrap();
    }

    let value = match conns.get(signal) {
        None => panic!("Got invalid signal"),
        Some(Connection::ValueInput(val, _)) => *val,
        Some(Connection::SignalInput(inp, _)) => get_signal_on(conns, inp, cache),
        Some(Connection::And(lhs, rhs, _)) => {
            let lhs = get_signal_on(conns, lhs, cache);
            let rhs = get_signal_on(conns, rhs, cache);
            lhs.bitand(rhs)
        }
        Some(Connection::AndValue(val, rhs, _)) => {
            let rhs = get_signal_on(conns, rhs, cache);
            val.bitand(rhs)
        }
        Some(Connection::Or(lhs, rhs, _)) => {
            let lhs = get_signal_on(conns, lhs, cache);
            let rhs = get_signal_on(conns, rhs, cache);
            lhs.bitor(rhs)
        }
        Some(Connection::LeftShift(lhs, val, _)) => {
            let lhs = get_signal_on(conns, lhs, cache);
            lhs.shl(val)
        }
        Some(Connection::RightShift(lhs, val, _)) => {
            let lhs = get_signal_on(conns, lhs, cache);
            lhs.shr(val)
        }
        Some(Connection::Not(lhs, _)) => {
            let lhs = get_signal_on(conns, lhs, cache);
            !lhs
        }
    };

    cache.insert(signal.to_string(), value);
    value
}

#[aoc(day7, part1)]
pub fn part1(conns: &HashMap<String, Connection>) -> u16 {
    let mut cache = HashMap::new();
    get_signal_on(conns, "a", &mut cache)
}

#[aoc(day7, part2)]
pub fn part2(conns: &HashMap<String, Connection>) -> u16 {
    let mut cache = HashMap::new();
    cache.insert(String::from("b"), 3176);
    get_signal_on(conns, "a", &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_p1() {
        let test_data = "123 -> x\n\
            456 -> y\n\
            x AND y -> d\n\
            x OR y -> e\n\
            x LSHIFT 2 -> f\n\
            y RSHIFT 2 -> g\n\
            NOT x -> h\n\
            NOT y -> i";

        let mut cache = HashMap::new();

        let conns = generate(test_data);
        assert_eq!(get_signal_on(&conns, "d", &mut cache), 72);
        assert_eq!(get_signal_on(&conns, "e", &mut cache), 507);
        assert_eq!(get_signal_on(&conns, "f", &mut cache), 492);
        assert_eq!(get_signal_on(&conns, "g", &mut cache), 114);
        assert_eq!(get_signal_on(&conns, "h", &mut cache), 65412);
        assert_eq!(get_signal_on(&conns, "i", &mut cache), 65079);
        assert_eq!(get_signal_on(&conns, "x", &mut cache), 123);
        assert_eq!(get_signal_on(&conns, "y", &mut cache), 456);
    }
}
