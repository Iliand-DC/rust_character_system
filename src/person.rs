pub mod person_stats;
pub use self::person_stats::*;

pub trait Doings {
    fn attack(&self) -> (String, i32);
    fn take_damage(&mut self, value:i32) -> String;
}

pub trait IStats {
    fn show_stats(&self) -> String;
}

// Here I changed it for better security using. 
// Now you need to call methods to get access for stats/bio/person variable.

impl Doings for Person {
    fn attack(&self) -> (String, i32) {
        (format!("{} нанёс {} урона\n",self.get_biography().name, self.get_stats().damage), self.get_stats().damage)
    }
    fn take_damage(&mut self, value:i32) -> String {
        let new_hp = self.get_stats().hp - value;
        self.set_stats(Stats {hp: new_hp, damage: self.get_stats().damage});
        format!("{} получил {} урона\n",self.get_biography().name, value)
    }
}

impl IStats for Person {
    fn show_stats(&self) -> String {
        format!("Имя: {}\nКласс: {}\nВозраст: {}\nРаса: {}\nУрон: {}\nЗдоровье: {}\n",
        self.get_biography().name,
        self.get_class(),
        self.get_biography().age,
        self.get_biography().race,
        self.get_stats().damage,
        self.get_stats().hp
        )
    }
}