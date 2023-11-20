fn main() {
    let jane_bio = Bio {
        name: "Jane Doe".to_string(),
        age: 26,
        race: "Human".to_string(),
    };
    let jane_stats = Stats {
        damage: 8,
        hp: 16
    };

    let mut jane = Person {
        class: "Paladin".to_string(),
        biography: jane_bio,
        stats: jane_stats,
    };

    let ork_bio = Bio {
        name: "Dumb Ork".to_string(),
        age: 42,
        race: "Ork".to_string(),
    };
    let ork_stats = Stats {
        damage: 12,
        hp: 25
    };
    let mut ork = Person {
        class: "Barbarian".to_string(),
        biography: ork_bio,
        stats: ork_stats
    };

    jane.show_stats();
    ork.show_stats();
    let (text_event, result_of_event) = jane.attack();
    println!("{}", text_event);
    println!("{}",ork.take_damage(result_of_event));

    let (text_event, result_of_event) = ork.attack();
    println!("{}", text_event);
    println!("{}", jane.take_damage(result_of_event));

    jane.show_stats();
    ork.show_stats();
}

struct  Stats {
    damage: i32,
    hp: i32,
}

struct Bio {
    name: String,
    race: String,
    age: i16,
}

struct Person {
    class: String,
    biography:Bio,
    stats:Stats,
}


trait Doings {
    fn attack(&self) -> (String, i32);
    fn take_damage(&mut self, value:i32) -> String;
}

trait IStats {
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