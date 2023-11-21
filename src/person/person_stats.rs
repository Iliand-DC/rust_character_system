mod chara_bio;
mod chara_stats;

pub use crate::person_stats::chara_bio::*;
pub use crate::person_stats::chara_stats::*;


// Here I deleted 'pub' just for better security.
// Now we have methods to get/set the varibales values :3
pub struct Person {
    class: String,
    biography:Bio,
    stats:Stats,
}

impl Person {
    pub fn get_class(&self) -> String {
        self.class.to_string()
    }
    pub fn get_biography(&self) -> Bio {
        Bio {
            name: self.biography.name.to_string(),
            race: self.biography.race.to_string(),
            age: self.biography.age,
        }
    }
    pub fn get_stats(&self) -> Stats {
        Stats {
            hp: self.stats.get_stat_hp(),
            damage: self.stats.get_stat_damage(),
        }
    }
    pub fn set_class(&mut self, class:String) {
        self.class = class
    }
    pub fn set_bio(&mut self, bio:Bio) {
        self.biography = bio
    }
    pub fn set_stats(&mut self, stats:Stats) {
        self.stats = stats
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