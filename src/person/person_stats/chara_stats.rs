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

// basic trait fn, and my practice to make some functions in class. If this has been in Rust initially, then
// I need to read more documentation and less do practice :P
// You can also practice if you want, to adding this simple fn in other classes or I do it myself
pub trait StatsBasicUsage {
    fn set_stat_damage(&mut self, new_damage:i32);
    fn set_stat_hp(&mut self, new_hp:i32);

    fn get_stat_damage(&self) -> i32;
    fn get_stat_hp(&self) -> i32;

    fn add_stat_hp(&mut self, delta_hp:i32); // This is not unsigned, then it can be used to remove hp
}

impl StatsBasicUsage for Stats {
    fn set_stat_damage(&mut self, new_damage:i32) {
        self.damage = new_damage;
    }
    fn set_stat_hp(&mut self, new_hp:i32) {
        self.hp = new_hp;
    }
    fn get_stat_damage(&self) -> i32 {
        self.damage
    }
    fn get_stat_hp(&self) -> i32 {
        self.hp
    }
    fn add_stat_hp(&mut self, delta_hp:i32) {
        self.set_stat_hp(self.hp + delta_hp);
    }
}