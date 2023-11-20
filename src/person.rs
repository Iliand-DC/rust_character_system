pub struct  Stats {
    damage: i32,
    hp: i32,
}

pub fn build_stats(damage_value: i32, hp_value:i32) -> Stats {
    Stats {
        damage: damage_value,
        hp: hp_value,
    }
}

pub struct Bio {
    name: String,
    race: String,
    age: i16,
}
pub fn build_bio(name_value:String, race_value:String, 
    age_value:i16) -> Bio {
    Bio {
        name: name_value,
        race: race_value,
        age:age_value,
    }
}

pub struct Person {
    class: String,
    biography:Bio,
    stats:Stats,
}
pub fn build_person(class_value: String, 
    biography_value: Bio, stats_value: Stats) -> Person {
        Person {
            class: class_value,
            biography: biography_value,
            stats: stats_value,
        }
    }

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