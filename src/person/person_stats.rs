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

#[allow(dead_code)]
impl Person {
    pub fn get_class(&self) -> String {
        self.class.to_string()
    }
    pub fn get_biography(&self) -> Bio {
        self.biography.get_biography()
    }
    pub fn get_stats(&self) -> Stats {
        self.stats.get_stats()
    }
    pub fn set_class(&mut self, class:String) {
        self.class = class
    }
    pub fn set_bio(&mut self, bio:Bio) {
        self.biography.set_biography(bio)
    }
    pub fn set_stats(&mut self, stats:Stats) {
        self.stats.set_stats(stats)
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