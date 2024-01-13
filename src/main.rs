mod person;
use crate::person::*;
mod sqlite_basic;
use crate::sqlite_basic::*;

fn main() {
    let jane_bio: Bio = build_bio(
        "Jane Doe".to_string(), 
        "Human".to_string(),
        25);

    let jane_stats: Stats = build_stats(
        8, 
        16);

    let mut jane = build_person(
        "Paladin".to_string(),
        jane_bio, 
        jane_stats);

    let ork_bio = build_bio(
        "Dumb Ork".to_string(),
        "Ork".to_string(), 
        42);
    
    let ork_stats = build_stats(12, 25);

    let mut ork = build_person(
        "Barbarian".to_string(), 
        ork_bio, 
        ork_stats);

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