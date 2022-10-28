use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{iproduct, Itertools};
use parse_display::{Display, FromStr};

#[derive(Copy, Clone, Debug)]
struct Item {
    cost: u32,
    dmg: u32,
    armor: u32,
}

/*
Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0
*/
fn generate_weapons() -> Vec<Item> {
    [(8, 4), (10, 5), (25, 6), (40, 7), (74, 8)]
        .iter()
        .fold(Vec::new(), |mut acc, &(cost, dmg)| {
            acc.push(Item {
                cost,
                dmg,
                armor: 0,
            });
            acc
        })
}

/*
Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5
*/
fn generate_armor() -> Vec<Item> {
    [(13, 1), (31, 2), (53, 3), (75, 4), (102, 5)].iter().fold(
        Vec::new(),
        |mut acc, &(cost, armor)| {
            acc.push(Item {
                cost,
                dmg: 0,
                armor,
            });
            acc
        },
    )
}

/*
Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3
*/
fn generate_rings() -> Vec<Item> {
    [
        (25, 1, 0),
        (50, 2, 0),
        (100, 3, 0),
        (20, 0, 1),
        (40, 0, 2),
        (80, 0, 3),
    ]
    .iter()
    .fold(Vec::new(), |mut acc, &(cost, dmg, armor)| {
        acc.push(Item { cost, dmg, armor });
        acc
    })
}

#[derive(Display, FromStr, Copy, Clone, Debug)]
#[display("Hit Points: {hp}\nDamage: {dmg}\nArmor: {armor}")]
pub struct EnemyStats {
    hp: u32,
    dmg: u32,
    armor: u32,
}

#[aoc_generator(day21)]
pub fn generate(inp: &str) -> Option<EnemyStats> {
    inp.parse::<EnemyStats>().ok()
}

fn is_winning(items: &[Item], mut own_hp: u32, mut enemy: EnemyStats) -> bool {
    let own_dmg = items.iter().map(|it| it.dmg).sum::<u32>();
    let own_arm = items.iter().map(|it| it.armor).sum::<u32>();

    loop {
        let dmg_dealt = own_dmg.checked_sub(enemy.armor).unwrap_or(1);
        enemy.hp = enemy.hp.saturating_sub(dmg_dealt);
        if enemy.hp == 0 {
            return true;
        }

        let dmg_taken = enemy.dmg.checked_sub(own_arm).unwrap_or(1);
        own_hp = own_hp.saturating_sub(dmg_taken);
        if own_hp == 0 {
            return false;
        }
    }
}

fn generate_combinations(max_amount: usize, items: &[Item]) -> Vec<Vec<Item>> {
    (0..=max_amount).fold(vec![], |acc, len| {
        items
            .iter()
            .copied()
            .combinations(len)
            .fold(acc, |mut acc, it| {
                acc.push(it);
                acc
            })
    })
}

fn get_item_combinations() -> Vec<Vec<Item>> {
    // 1 Weapon
    let weapons = generate_weapons();
    // 0-1 Armor
    let armor = generate_combinations(1, &generate_armor());
    // 0-2 Rings
    let rings = generate_combinations(2, &generate_rings());

    iproduct!(weapons, armor, rings)
        .map(|(w, a, r)| {
            vec![w]
                .iter()
                .chain(a.iter())
                .chain(r.iter())
                .copied()
                .collect_vec()
        })
        .collect_vec()
}

#[aoc(day21, part1)]
pub fn part1(inp: &EnemyStats) -> Option<u32> {
    get_item_combinations()
        .iter()
        .filter(|it| is_winning(it, 100, *inp))
        .map(|it| it.iter().map(|i| i.cost).sum())
        .min()
}

#[aoc(day21, part2)]
pub fn part2(inp: &EnemyStats) -> Option<u32> {
    get_item_combinations()
        .iter()
        .filter(|it| !is_winning(it, 100, *inp))
        .map(|it| it.iter().map(|i| i.cost).sum())
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_p1() {
        // For example, suppose you have 8 hit points, 5 damage, and 5 armor,
        // and that the boss has 12 hit points, 7 damage, and 2 armor:
        let items = vec![Item {
            cost: 0,
            dmg: 5,
            armor: 5,
        }];

        assert!(is_winning(
            &items,
            8,
            EnemyStats {
                hp: 12,
                dmg: 7,
                armor: 2,
            },
        ));
    }
}
