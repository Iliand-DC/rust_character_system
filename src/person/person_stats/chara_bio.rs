pub struct Bio {
    pub name: String,
    pub race: String,
    pub age: i16,
}

pub fn build_bio(name:String, race:String, 
    age:i16) -> Bio {
    Bio {
        name,
        race,
        age,
    }
}
