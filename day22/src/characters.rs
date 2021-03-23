use std::cmp::max;

#[derive(Clone)]
pub struct Player {
    pub hp: u32,
    pub def: u32,
    pub mana: u32,
}

impl Player {
    pub fn guard(&mut self, dmg: u32) {
        self.hp = self.hp.saturating_sub(max(1, dmg - self.def));
    }
}

#[derive(Clone)]
pub struct Boss {
    pub hp: u32,
    pub dmg: u32,
}

impl Boss {
    pub fn attack(&self, player: &mut Player) {
        player.guard(self.dmg)
    }

    pub fn guard(&mut self, dmg: u32) {
        self.hp = self.hp.saturating_sub(dmg);
    }
}
