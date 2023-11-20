fn main() {
    let mut character = Person {
        name: "Jane Doe".to_string(),
        class: "Paladin".to_string(),
        age: 26,
        damage: 8,
        hp: 16
    };
    character.show_stats();
    println!("{}", character.attack());
    println!("{}", character.take_damage(5));
    character.show_stats();

}

struct Person {
    name: String,
    class: String,
    age: i16,
    damage: i32,
    hp: i32,
}

trait Doings {
    fn attack(&self) -> String;
    fn take_damage(&mut self, value:i32) -> String;
}

trait Print {
    fn show_stats(&self);
}

impl Doings for Person {
    fn attack(&self) -> String {
        format!("Ты нанёс {} урона\n", self.damage)
    }
    fn take_damage(&mut self, value:i32) -> String {
        self.hp = self.hp - value;
        format!("Ты получил {} урона\n", value)
    }
}

impl Print for Person {
    fn show_stats(&self) {
        println!("Имя: {}\nКласс: {}\nВозраст: {}\nУрон: {}\nЗдоровье: {}\n",
        self.name,
        self.class,
        self.age,
        self.damage,
        self.hp)
    }
}