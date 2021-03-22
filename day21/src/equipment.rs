use lazy_static::lazy_static;

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
lazy_static! {
    pub static ref WEAPONS: [Item; 5] = [
        Item::new("Dagger", 8, 4, 0),
        Item::new("Shortsword", 10, 5, 0),
        Item::new("Warhammer", 25, 6, 0),
        Item::new("Longsword", 40, 7, 0),
        Item::new("Greataxe", 74, 8, 0),
    ];
}

lazy_static! {
    pub static ref ARMOR: [Item; 6] = [
        Item::new("None", 0, 0, 0),
        Item::new("Leather", 13, 0, 1),
        Item::new("Chainmail", 31, 0, 2),
        Item::new("Splintmail", 53, 0, 3),
        Item::new("Bandedmail", 75, 0, 4),
        Item::new("Platemail", 102, 0, 5),
    ];
}

lazy_static! {
    pub static ref RINGS: [Item; 8] = [
        Item::new("None", 0, 0, 0),
        Item::new("None", 0, 0, 0),
        Item::new("Damage +1", 25, 1, 0),
        Item::new("Damage +2", 50, 2, 0),
        Item::new("Damage +3", 100, 3, 0),
        Item::new("Defense +1", 20, 0, 1),
        Item::new("Defense +2", 40, 0, 2),
        Item::new("Defense +3", 80, 0, 3),
    ];
}
