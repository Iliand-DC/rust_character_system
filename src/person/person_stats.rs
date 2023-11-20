
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