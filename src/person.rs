pub mod person_stats;

use crate::person::person_stats::*;

pub trait Doings {
    fn attack(&self) -> (String, i32);
    fn take_damage(&mut self, value:i32) -> String;
}

pub trait IStats {
    fn show_stats(&self);
}

impl Doings for Person {
    fn attack(&self) -> (String, i32) {
        (format!("{} нанёс {} урона\n",self.biography.name, self.stats.damage), self.stats.damage)
    }
    fn take_damage(&mut self, value:i32) -> String {
        self.stats.hp = self.stats.hp - value;
        format!("{} получил {} урона\n",self.biography.name, value)
    }
}

impl IStats for Person {
    fn show_stats(&self) {
        println!("Имя: {}\nКласс: {}\nВозраст: {}\nРаса: {}\nУрон: {}\nЗдоровье: {}\n",
        self.biography.name,
        self.class,
        self.biography.age,
        self.biography.race,
        self.stats.damage,
        self.stats.hp
        )
    }
}