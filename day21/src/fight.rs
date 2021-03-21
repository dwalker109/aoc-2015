use crate::character::CharacterType;

#[derive(Debug)]
pub struct Item {
    name: String,
    pub cost: u32,
    pub dmg: u32,
    pub def: u32,
}

impl Item {
    pub fn new(name: &str, cost: u32, dmg: u32, def: u32) -> Item {
        Item {
            name: name.to_string(),
            cost,
            dmg,
            def,
        }
    }
}

#[derive(Debug)]
pub struct Outcome {
    pub winner: CharacterType,
    pub gold_spent: u32,
}
