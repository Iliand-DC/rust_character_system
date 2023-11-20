mod chara_bio;
mod chara_stats;

pub use crate::person_stats::chara_bio::*;
pub use crate::person_stats::chara_stats::*;

pub struct Person {
    pub class: String,
    pub biography:Bio,
    pub stats:Stats,
}

pub fn build_person(class: String, 
    biography: Bio, stats: Stats) -> Person {
        Person {
            class,
            biography,
            stats,
        }
    }