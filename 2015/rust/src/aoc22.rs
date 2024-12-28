use std::cmp::min;
use std::io;

enum Action {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge
}

#[derive(Clone, Debug)]
enum Effect {
    Shield,
    Poison,
    Recharge
}

struct OutOfManaError {
}

struct Player {
    hp: usize,
    mana: usize,
    armor: usize,
    effects: Vec<Effect>
}

#[derive(Clone, Debug)]
pub struct Entity {
    hp: usize,
    damage: usize,
    effects: Vec<Effect>
}

const BOSS: Entity = Entity {
    hp: 71,
    damage: 10,
    effects: vec![]
};

impl Player {
    fn new() -> Self {
        Self {
            hp: 50,
            mana: 500,
            armor: 0,
            effects: vec![]
        }
    }

    fn act(&mut self, input: Action, boss: &mut Entity) -> Result<(), OutOfManaError>{
        //apply effects
        for e in self.effects.iter_mut() {
            e.apply(&mut self);
        }
        self.effects = self.effects.into_iter().filter(|&e| e.turns > 0).collect(); 
        let choice;
        match choice {
            Action::MagicMissile => {
                self.mana = self.mana.checked_sub(53).ok_or(OutOfManaError{})?;
                boss.hp = boss.hp.checked_sub(4).unwrap_or(0);
            },
            Action::Drain => {
                self.mana = self.mana.checked_sub(73)?.ok_or(OutOfManaError{})?;
                boss.hp = boss.hp.checked_sub(2).unwrap_or(0);
                self.hp += 2;
            },
            Action::Shield => {
                self.mana = self.mana.checked_sub(113).ok_or(OutOfManaError{})?;
                self.effects.push(Effect::Shield);
            },
            Action::Poison => {
                self.mana = self.mana.checked_sub(113).ok_or(OutOfManaError{})?;
                boss.effects.push(Effect::Poison);
            },
            Action::Recharge => {
                self.mana = self.mana.checked_sub(229).ok_or(OutOfManaError{})?;
                self.effects.push(Effect::Recharge);
            }

        }
    }
}

impl<&mut Player> Effect<&mut Player> {
    fn apply(&mut self, player: &mut Player) {
        match self {
            Self::Shield => {
                self.turns -= 1;
                player.armor += 7;
            },
            Self::Recharge => {
                self.turns -= 1;
                player.mana += 101;
            },
            _ => {}
        }
    }
}

impl<Entity> Effect<Entity> {
    fn apply(&mut self, entity: &mut Entity) {
        match self {
            Self::Poison => {
                self.turns -= 1;
                entity.hp = entity.hp.checked_sub(3).unwrap_or(0);
            },
            _ => {}
        }
    }
}

fn play(player: &mut Player, boss: &mut Entity) -> bool {
    while player.hp > 0 && player.mana > 0 && boss.hp > 0 {
        let action = match &io::read_to_string(io::stdin()).unwrap()[..] {
            "M" => Action::MagicMissile,
            "D" => Action::Drain,
            "S" => Action::Shield,
            "P" => Action::Poison,
            "R" => Action::Recharge,
            _ => {panic!("not a valid message");}
        };
        player.act(action, boss);
        player.hp -= min(
            player.hp,
            boss.damage.checked_sub(player.armor).unwrap_or(1),
        );
    }
    player.hp > 0 && player.mana > 0 
}

pub fn get_input() -> Entity {
    BOSS
}

pub fn part_1(boss: &Entity) -> usize {
    let mut player = Player::new();
    let mut boss = boss.clone();
    if play(&mut player, &mut boss) {
        player.mana
    } else {
        0
    }
}
