use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{Display, FromStr};
use std::collections::HashSet;

#[derive(FromStr, Display, Copy, Clone, Debug)]
#[display("Hit Points: {hp}\nDamage: {dmg}")]
pub struct Enemy {
    hp: u32,
    dmg: u32,

    #[from_str(default)]
    poison: u32,
}

#[derive(Copy, Clone, Debug)]
struct Player {
    hp: u32,
    mana: u32,
    dot: u32,
    armor: u32,
    recharge: u32,
    shield: u32,
}

impl Player {
    const fn new(hp: u32, mana: u32, dot: u32) -> Self {
        Self {
            hp,
            mana,
            dot,
            armor: 0,
            recharge: 0,
            shield: 0,
        }
    }
}

enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

const SPELLS: [Spell; 5] = [
    Spell::Recharge,
    Spell::MagicMissile,
    Spell::Drain,
    Spell::Shield,
    Spell::Poison,
];

#[aoc_generator(day22)]
pub fn generate(inp: &str) -> Option<Enemy> {
    inp.parse::<Enemy>().ok()
}

fn apply_effects(player: &mut Player, enemy: &mut Enemy) {
    if player.recharge > 0 {
        player.mana += 101;
        player.recharge -= 1;
    }

    if player.shield > 0 {
        player.shield -= 1;

        if player.shield == 0 {
            player.armor = 0;
        }
    }

    if enemy.poison > 0 {
        enemy.hp = enemy.hp.saturating_sub(3);
        enemy.poison -= 1;
    }
}

fn do_battle(
    mut player: Player,
    mut enemy: Enemy,
    player_turn: bool,
    spent_mana: u32,
    cache: &mut HashSet<u32>,
) {
    if player_turn {
        player.hp = player.hp.saturating_sub(player.dot);
    }

    if player.hp == 0 {
        return;
    }

    apply_effects(&mut player, &mut enemy);

    if enemy.hp == 0 {
        cache.insert(spent_mana);
        return;
    }

    if player_turn {
        if player.mana < 53 {
            return;
        }

        for spell in SPELLS {
            if matches!(spell, Spell::Shield) && player.shield > 0 {
                continue;
            }
            if matches!(spell, Spell::Recharge) && player.recharge > 0 {
                continue;
            }
            if matches!(spell, Spell::Poison) && enemy.poison > 0 {
                continue;
            }

            match spell {
                Spell::MagicMissile => {
                    let mut new_player = player;
                    new_player.mana -= 53;
                    let mut new_enemy = enemy;
                    new_enemy.hp = new_enemy.hp.saturating_sub(4);
                    do_battle(new_player, new_enemy, false, spent_mana + 53, cache);
                }
                Spell::Drain => {
                    if let Some(m) = player.mana.checked_sub(73) {
                        let mut new_player = player;
                        new_player.mana = m;
                        new_player.hp += 2;
                        let mut new_enemy = enemy;
                        new_enemy.hp = new_enemy.hp.saturating_sub(2);
                        do_battle(new_player, new_enemy, false, spent_mana + 73, cache);
                    } else {
                        continue;
                    }
                }
                Spell::Shield => {
                    if let Some(m) = player.mana.checked_sub(113) {
                        let mut new_player = player;
                        new_player.mana = m;
                        new_player.shield = 6;
                        new_player.armor += 7;
                        do_battle(new_player, enemy, false, spent_mana + 113, cache);
                    } else {
                        continue;
                    }
                }
                Spell::Poison => {
                    if let Some(m) = player.mana.checked_sub(173) {
                        let mut new_player = player;
                        new_player.mana = m;

                        let mut new_enemy = enemy;
                        new_enemy.poison = 6;
                        do_battle(new_player, new_enemy, false, spent_mana + 173, cache);
                    } else {
                        continue;
                    }
                }
                Spell::Recharge => {
                    if let Some(m) = player.mana.checked_sub(229) {
                        let mut new_player = player;
                        new_player.mana = m;
                        new_player.recharge = 5;
                        do_battle(new_player, enemy, false, spent_mana + 229, cache);
                    } else {
                        continue;
                    }
                }
            };
        }
    } else {
        player.hp = player.hp.saturating_sub(enemy.dmg - player.armor);
        do_battle(player, enemy, true, spent_mana, cache);
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[aoc(day22, part1)]
pub fn part1(enemy: &Enemy) -> Option<u32> {
    let mut cache = HashSet::new();

    let player = Player::new(50, 500, 0);

    do_battle(player, *enemy, true, 0, &mut cache);

    cache.iter().min().copied()
}

#[allow(clippy::trivially_copy_pass_by_ref)]
#[aoc(day22, part2)]
pub fn part2(enemy: &Enemy) -> Option<u32> {
    let mut cache = HashSet::new();

    let player = Player::new(50, 500, 1);

    do_battle(player, *enemy, true, 0, &mut cache);

    cache.iter().min().copied()
}
