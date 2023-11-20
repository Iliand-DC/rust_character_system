fn main() {
    let mut jane = Person {
        name: "Jane Doe".to_string(),
        class: "Paladin".to_string(),
        age: 26,
        damage: 8,
        hp: 16
    };

    let mut ork = Person {
        name: "Dumb Ork".to_string(),
        class: "Barbarian".to_string(),
        age: 42,
        damage: 12,
        hp: 25
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

struct Person {
    name: String,
    class: String,
    age: i16,
    damage: i32,
    hp: i32,
}

trait Doings {
    fn attack(&self) -> (String, i32);
    fn take_damage(&mut self, value:i32) -> String;
}

trait Stats {
    fn show_stats(&self);
}

impl Doings for Person {
    fn attack(&self) -> (String, i32) {
        (format!("{} нанёс {} урона\n",self.name, self.damage), self.damage)
    }
    fn take_damage(&mut self, value:i32) -> String {
        self.hp = self.hp - value;
        format!("{} получил {} урона\n",self.name, value)
    }
}

impl Stats for Person {
    fn show_stats(&self) {
        println!("Имя: {}\nКласс: {}\nВозраст: {}\nУрон: {}\nЗдоровье: {}\n",
        self.name,
        self.class,
        self.age,
        self.damage,
        self.hp)
    }
}