use crate::equipment::Item;

#[derive(Debug, Clone)]
pub struct Character {
    character_type: CharacterType,
    hp: u32,
    base_dmg: u32,
    base_def: u32,
    wpn: Option<&'static Item>,
    arm: Option<&'static Item>,
    rng: Option<[&'static Item; 2]>,
}

impl Character {
    pub fn new(
        character_type: CharacterType,
        hp: u32,
        base_dmg: u32,
        base_def: u32,
        wpn: Option<&'static Item>,
        arm: Option<&'static Item>,
        rng: Option<[&'static Item; 2]>,
    ) -> Character {
        Character {
            character_type,
            hp,
            base_dmg,
            base_def,
            wpn,
            arm,
            rng,
        }
    }

    pub fn attack(&self, other: &mut Character) {
        other.hp = other
            .hp
            .saturating_sub(self.dmg().saturating_sub(other.def()));
    }

    pub fn is_player(&self) -> bool {
        self.character_type == CharacterType::Player
    }

    pub fn is_defeated(&self) -> bool {
        self.hp == 0
    }

    fn dmg(&self) -> u32 {
        let wpn_dmg = match self.wpn {
            Some(w) => w.dmg,
            None => 0,
        };

        let rng_dmg = match self.rng {
            Some(r) => r[0].dmg + r[1].dmg,
            None => 0,
        };

        self.base_dmg + wpn_dmg + rng_dmg
    }

    fn def(&self) -> u32 {
        let arm_def = match self.arm {
            Some(a) => a.def,
            None => 0,
        };

        let rng_def = match self.rng {
            Some(r) => r[0].def + r[1].def,
            None => 0,
        };

        self.base_def + arm_def + rng_def
    }

    pub fn gold_spent(&self) -> u32 {
        self.wpn.unwrap().cost
            + self.arm.unwrap().cost
            + self.rng.unwrap().iter().fold(0, |acc, &r| acc + r.cost)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CharacterType {
    Player,
    Boss,
}
