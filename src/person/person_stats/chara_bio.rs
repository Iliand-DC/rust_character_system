pub struct Bio {
    pub name: String,
    pub race: String,
    pub age: i16,
}

#[allow(dead_code)]
impl Bio {
    pub fn get_biography(&self) -> Bio {
        Bio { 
            name: self.name.to_string(), 
            race: self.race.to_string(), 
            age: self.age 
        }
    }

    pub fn set_biography(&mut self, new_biography:Bio) {
        self.name = new_biography.name;
        self.race = new_biography.race;
        self.age = new_biography.age;
    }

    pub fn get_biography_name(&self) -> String {
        self.name.to_string()
    }
    pub fn get_biography_race(&self) -> String {
        self.race.to_string()
    }
    pub fn get_biography_age(&self) -> i16 {
        self.age
    }

    pub fn set_biography_name(&mut self, new_name:String) {
        self.name = new_name
    }
    pub fn set_biography_race(&mut self, new_race:String) {
        self.race = new_race
    }
    pub fn set_biography_age(&mut self, new_age: i16) {
        self.age = new_age
    }
}

pub fn build_bio(name:String, race:String, 
    age:i16) -> Bio {
    Bio {
        name,
        race,
        age,
    }
}
