use std::io;

pub fn get_playername() {
    println!("Enter a username!");
    let mut playername = String::new();

    io::stdin()
        .read_line(&mut playername);

    println!("Is this name correct? -> {playername}");
    acceptance();
}

pub fn acceptance(playername: &String) -> usize {
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("LINE ERROR");
    match playername.cmp(&response) {
        println!("After");
    }
}
