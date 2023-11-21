pub struct  Stats {
    pub damage: i32,
    pub hp: i32,
}

pub fn build_stats(damage: i32, hp:i32) -> Stats {
    Stats {
        damage,
        hp,
    }
}

// Sorry, I changed your trait cause this wasn't right way to create functions for Stats class
// You can use 'impl Class_name' for initial class methods.
#[allow(dead_code)]
impl Stats {
    pub fn get_stats(&self) -> Stats {
        Stats {
            damage: self.damage,
            hp: self.hp
        }
    }

    pub fn set_stats(&mut self, new_stats:Stats) {
        self.damage = new_stats.damage;
        self.hp = new_stats.hp;
    }

    pub fn set_stat_damage(&mut self, new_damage:i32) {
        self.damage = new_damage;
    }
    pub fn set_stat_hp(&mut self, new_hp:i32) {
        self.hp = new_hp;
    }
    pub fn get_stat_damage(&self) -> i32 {
        self.damage
    }
    pub fn get_stat_hp(&self) -> i32 {
        self.hp
    }
    pub fn add_stat_hp(&mut self, delta_hp:i32) {
        self.set_stat_hp(self.hp + delta_hp);
    }
}