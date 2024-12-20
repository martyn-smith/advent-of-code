use itertools::Itertools;
use std::cmp::min;

#[derive(Clone, Copy, Debug)]
enum Weapon {
    Dagger,     //   8     4       0
    Shortsword, //  10     5       0
    Warhammer,  //  25     6       0
    Longsword,  //  40     7       0
    Greataxe,   //  74     8       0
}

#[derive(Clone, Copy, Debug)]
enum Armor {
    Leather,    //  13     0       1
    Chainmail,  //  31     0       2
    Splintmail, //  53     0       3
    Bandedmail, //  75     0       4
    Platemail,  // 102     0       5
    None,
}

#[derive(Clone, Copy, Debug)]
enum Ring {
    Damage1,  //  25     1       0
    Damage2,  //  50     2       0
    Damage3,  // 100     3       0
    Defense1, //  20     0       1
    Defense2, //  40     0       2
    Defense3, //  80     0       3
    None,
}

const WEAPONS: [Weapon; 5] = [
    Weapon::Dagger,
    Weapon::Shortsword,
    Weapon::Warhammer,
    Weapon::Longsword,
    Weapon::Greataxe,
];

const ARMORS: [Armor; 6] = [
    Armor::Leather,
    Armor::Chainmail,
    Armor::Splintmail,
    Armor::Bandedmail,
    Armor::Platemail,
    Armor::None,
];

const RINGS: [Ring; 6] = [
    Ring::Damage1,
    Ring::Damage2,
    Ring::Damage3,
    Ring::Defense1,
    Ring::Defense2,
    Ring::Defense3,
];

#[derive(PartialEq)]
enum GameResult {
    PlayerWins,
    BossWins,
}

#[derive(Clone, Debug)]
struct Loadout {
    weapon: Weapon,
    armor: Armor,
    rings: Vec<Ring>,
    cost: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct Entity {
    hp: usize,
    damage: usize,
    armor: usize,
}

struct Player {
    entity: Entity,
    loadout: Loadout,
}

const BOSS: Entity = Entity {
    hp: 109,
    damage: 8,
    armor: 2,
};

impl Weapon {
    fn cost(&self) -> usize {
        match self {
            Self::Dagger => 8,
            Self::Shortsword => 10,
            Self::Warhammer => 25,
            Self::Longsword => 40,
            Self::Greataxe => 74,
        }
    }

    fn atk(&self) -> usize {
        match self {
            Self::Dagger => 4,
            Self::Shortsword => 5,
            Self::Warhammer => 6,
            Self::Longsword => 7,
            Self::Greataxe => 8,
        }
    }
}

impl Armor {
    fn cost(&self) -> usize {
        match self {
            Self::Leather => 13,
            Self::Chainmail => 31,
            Self::Splintmail => 53,
            Self::Bandedmail => 75,
            Self::Platemail => 102,
            Self::None => 0,
        }
    }

    fn defense(&self) -> usize {
        match self {
            Self::Leather => 1,
            Self::Chainmail => 2,
            Self::Splintmail => 3,
            Self::Bandedmail => 4,
            Self::Platemail => 5,
            Self::None => 0,
        }
    }
}

impl Ring {
    fn cost(&self) -> usize {
        match self {
            Self::Damage1 => 25,
            Self::Damage2 => 50,
            Self::Damage3 => 100,
            Self::Defense1 => 20,
            Self::Defense2 => 40,
            Self::Defense3 => 80,
            Self::None => 0,
        }
    }

    fn atk(&self) -> usize {
        match self {
            Self::Damage1 => 1,
            Self::Damage2 => 2,
            Self::Damage3 => 3,
            Self::Defense1 => 0,
            Self::Defense2 => 0,
            Self::Defense3 => 0,
            Self::None => 0,
        }
    }

    fn defense(&self) -> usize {
        match self {
            Self::Damage1 => 0,
            Self::Damage2 => 0,
            Self::Damage3 => 0,
            Self::Defense1 => 1,
            Self::Defense2 => 2,
            Self::Defense3 => 3,
            Self::None => 0,
        }
    }
}

impl Loadout {
    fn new(weapon: Weapon, armor: Armor, rings: Vec<Ring>) -> Self {
        let cost = weapon.cost() + armor.cost() + rings.iter().map(|r| r.cost()).sum::<usize>();
        Self {
            weapon,
            armor,
            rings,
            cost,
        }
    }
}

impl Player {
    fn new(loadout: Loadout) -> Self {
        let hp = 100;
        let damage = loadout.weapon.atk() + loadout.rings.iter().map(|r| r.atk()).sum::<usize>();
        let armor =
            loadout.armor.defense() + loadout.rings.iter().map(|r| r.defense()).sum::<usize>();
        let entity = Entity { hp, damage, armor };
        Self { entity, loadout }
    }
}

fn play(player: &mut Entity, boss: &mut Entity) -> GameResult {
    while player.hp > 0 && boss.hp > 0 {
        boss.hp -= min(boss.hp, player.damage.checked_sub(boss.armor).unwrap_or(1));
        player.hp -= min(
            player.hp,
            boss.damage.checked_sub(player.armor).unwrap_or(1),
        );
    }
    if player.hp > 0 {
        GameResult::PlayerWins
    } else {
        GameResult::BossWins
    }
}

pub fn get_input() -> Entity {
    BOSS
}

pub fn part_1(boss: &Entity) -> usize {
    WEAPONS
        .iter()
        .cartesian_product(ARMORS.iter())
        .cartesian_product(
            vec![]
                .into_iter()
                .chain(RINGS.clone().iter().map(|&r| vec![r]))
                .chain(
                    RINGS
                        .iter()
                        .combinations(2)
                        .map(|r| r.into_iter().cloned().collect()),
                ),
        )
        .filter_map(|((&w, &a), r)| {
            let loadout = Loadout::new(w, a, r);
            let mut player = Player::new(loadout);
            let mut boss = boss.clone();
            if play(&mut player.entity, &mut boss) == GameResult::PlayerWins {
                Some(player.loadout.cost)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

pub fn part_2(boss: &Entity) -> usize {
    WEAPONS
        .iter()
        .cartesian_product(ARMORS.iter())
        .cartesian_product(
            vec![]
                .into_iter()
                .chain(RINGS.clone().iter().map(|&r| vec![r]))
                .chain(
                    RINGS
                        .iter()
                        .combinations(2)
                        .map(|r| r.into_iter().cloned().collect()),
                ),
        )
        .filter_map(|((&w, &a), r)| {
            let loadout = Loadout::new(w, a, r);
            let mut player = Player::new(loadout);
            let mut boss = boss.clone();
            if play(&mut player.entity, &mut boss) == GameResult::BossWins {
                Some(player.loadout.cost)
            } else {
                None
            }
        })
        .max()
        .unwrap()
}
