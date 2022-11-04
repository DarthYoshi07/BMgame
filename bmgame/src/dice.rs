use rand::Rng;
use std::io;

pub fn other_file() {
    /* let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input);
    
    println!("You typed: {}", input.trim());
*/
    d20();
}

pub fn d20() {
    let dice: &str = "D20";
    let roll = rand::thread_rng().gen_range(1..=20);
    let rolled: bool = false;
    println!("(Current die: {dice})");
    println!("You rolled a: {roll}");
}

pub fn usr_roll() {
    println!("I am usr roll!");
}
