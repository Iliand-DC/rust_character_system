pub mod person;
use crate::person::*;

/*
Here your old test. Don't worry, I didn't delete anything from your test ≽^·⩊·^≼ 
I commented it because it's don't used now and it calling warning while it uncommented.

fn testbench1(mut person_1:Person, mut person_2:Person) {
    let (text_event, result_of_event) = person_1.attack();
    println!("{}", text_event);
    println!("{}",person_2.take_damage(result_of_event));

    let (text_event, result_of_event) = person_2.attack();
    println!("{}", text_event);
    println!("{}", person_1.take_damage(result_of_event));

    person_1.show_stats();
    person_2.show_stats();
}
*/

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

    // essence of this test check of working fn in chara_stats.rs file. I need be confident that's it's working.
    // ah yes, if you want to activate your last test, be carefull, I don't know, but after using testbench1 fn
    // fields in "ork" and "jane" didn't want to be used.
    let x_dmg = ork.stats.get_stat_damage();
    let mut x_hp = ork.stats.get_stat_hp();
    let x_name = ork.biography.name;
    println!("{x_name} have {x_dmg} dmg and {x_hp} health!");

    ork.stats.set_stat_hp(50);
    x_hp = ork.stats.get_stat_hp();
    println!("{x_name} have {x_dmg} dmg and {x_hp} health!");

    let mut y_dmg = jane.stats.get_stat_damage();
    let y_hp = jane.stats.get_stat_hp();
    let y_name = jane.biography.name;
    println!("{y_name} have {y_dmg} dmg and {y_hp} health!");

    jane.stats.set_stat_damage(10);
    y_dmg = jane.stats.get_stat_damage();
    println!("{y_name} have {y_dmg} dmg and {y_hp} health!");


}