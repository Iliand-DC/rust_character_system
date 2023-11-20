
pub struct  Stats {
    pub damage: i32,
    pub hp: i32,
}

pub struct Bio {
    pub name: String,
    pub race: String,
    pub age: i16,
}


pub struct Person {
    pub class: String,
    pub biography:Bio,
    pub stats:Stats,
}


pub fn build_stats(damage: i32, hp:i32) -> Stats {
    Stats {
        damage,
        hp,
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

pub fn build_person(class: String, 
    biography: Bio, stats: Stats) -> Person {
        Person {
            class,
            biography,
            stats,
        }
    }