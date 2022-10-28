use aoc_runner_derive::{aoc, aoc_generator};
use serde_json::{Map, Value};

#[aoc_generator(day12)]
pub fn generate(inp: &str) -> Value {
    inp.parse().unwrap()
}

fn is_red(val: &Value) -> bool {
    val.as_str().map_or(false, |s| s.eq("red"))
}

fn is_red_object(obj: &Map<String, Value>) -> bool {
    obj.values().any(is_red)
}

fn sum_numbers(json: &Value, include_red: bool) -> i64 {
    match json {
        Value::Number(n) => n.as_i64().unwrap_or_default(),
        Value::Array(a) => a.iter().map(|it| sum_numbers(it, include_red)).sum(),
        Value::Object(o) if include_red || !is_red_object(o) => {
            o.values().map(|it| sum_numbers(it, include_red)).sum()
        }
        Value::Object(_) | Value::Null | Value::Bool(_) | Value::String(_) => 0,
    }
}

#[aoc(day12, part1)]
pub fn part1(inp: &Value) -> i64 {
    sum_numbers(inp, true)
}

#[aoc(day12, part2)]
pub fn part2(inp: &Value) -> i64 {
    sum_numbers(inp, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input_p1() {
        let test_data = vec![
            ("[1,2,3]", 6),
            ("{\"a\": 2, \"b\": 4}", 6),
            ("[[3]]", 3),
            ("{\"a\":{\"b\":4},\"c\":-1}", 3),
        ];

        for (inp, expected) in test_data {
            let json = generate(inp);
            let res = part1(&json);
            assert_eq!(res, expected);
        }
    }

    #[test]
    fn test_sample_input_p2() {
        let test_data = vec![
            ("[1,2,3]", 6),
            ("{\"a\": 2, \"b\": 4}", 6),
            ("[1, {\"c\":\"red\",\"b\":2},3]", 4),
            ("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}", 0),
        ];

        for (inp, expected) in test_data {
            let json = generate(inp);
            let res = part2(&json);
            assert_eq!(res, expected);
        }
    }
}
