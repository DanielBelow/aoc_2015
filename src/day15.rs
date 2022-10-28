use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;
use parse_display::{Display, FromStr};
use std::collections::HashMap;

#[derive(Display, FromStr, Clone, Debug)]
#[display("capacity {capacity}, durability {durability}, flavor {flavor}, texture {texture}, calories {calories}")]
pub struct IngredientData {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

#[derive(Display, FromStr, Clone, Debug)]
#[display("{name}: {data}")]
struct Ingredient {
    name: String,
    data: IngredientData,
}

#[aoc_generator(day15)]
pub fn generate(inp: &str) -> HashMap<String, IngredientData> {
    inp.lines()
        .filter_map(|it| it.parse::<Ingredient>().ok())
        .map(|it| (it.name, it.data))
        .collect()
}

fn calculate_score(
    ingredients: &[(i64, &str)],
    data: &HashMap<String, IngredientData>,
    drop_if: fn(&[i64; 5]) -> bool,
) -> Option<i64> {
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    let mut calories = 0;

    for (num, key) in ingredients {
        let ing_data = data.get(*key)?;
        capacity += num * ing_data.capacity;
        durability += num * ing_data.durability;
        flavor += num * ing_data.flavor;
        texture += num * ing_data.texture;
        calories += num * ing_data.calories;
    }

    if drop_if(&[capacity, durability, flavor, texture, calories]) {
        return Some(0);
    }

    Some(capacity * durability * flavor * texture)
}

fn find_best_score(
    data: &HashMap<String, IngredientData>,
    pred: fn(&[i64; 5]) -> bool,
) -> Option<i64> {
    let mut result = 0;

    for (sp, pb, f) in iproduct!(0..=100, 0..=100, 0..=100) {
        let s = 100 - sp - pb - f;

        if sp + pb + f + s != 100 {
            continue;
        }

        let ingredients = &[
            (sp, "Sprinkles"),
            (pb, "PeanutButter"),
            (f, "Frosting"),
            (s, "Sugar"),
        ];

        let score = calculate_score(ingredients, data, pred)?;
        result = result.max(score);
    }

    Some(result)
}

fn part1_pred(elems: &[i64; 5]) -> bool {
    elems.iter().take(4).any(|e| *e < 0)
}

fn part2_pred(elems: &[i64; 5]) -> bool {
    const CALORIES: Option<&i64> = Some(&500);
    part1_pred(elems) || !elems.last().eq(&CALORIES)
}

#[aoc(day15, part1)]
pub fn part1(data: &HashMap<String, IngredientData>) -> Option<i64> {
    find_best_score(data, part1_pred)
}

#[aoc(day15, part2)]
pub fn part2(data: &HashMap<String, IngredientData>) -> Option<i64> {
    find_best_score(data, part2_pred)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_score(
        gen: &HashMap<String, IngredientData>,
        pred: fn(&[i64; 5]) -> bool,
    ) -> Option<i64> {
        (0..=100)
            .filter_map(|bs| {
                let c = 100 - bs;
                calculate_score(&[(bs, "Butterscotch"), (c, "Cinnamon")], gen, pred)
            })
            .max()
    }

    #[test]
    fn test_sample_input_p1() {
        let test_data = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\n\
                                Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

        let gen = generate(test_data);

        let res = get_test_score(&gen, part1_pred);
        assert_eq!(res.unwrap(), 62_842_880);
    }

    #[test]
    fn test_sample_input_p2() {
        let test_data = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8\n\
                                Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

        let gen = generate(test_data);

        let res = get_test_score(&gen, part2_pred);
        assert_eq!(res.unwrap(), 57_600_000);
    }
}
