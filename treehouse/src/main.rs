use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
	Self {
	    name: name.to_lowercase(),
	    greeting: greeting.to_string(),
	}
    }

    fn greet_visitor(&self) {
	println!("{0}", self.greeting);
    }
}

fn input_name() -> String {
    println!("Please enter your name!");
    let mut guest_name = String::new();
    stdin()
	.read_line(&mut guest_name)
	.expect("Failed to read line");
    guest_name
	.trim()
	.to_lowercase()
}

fn main() {
    let name: String = input_name();

    let visitor_list = [
	Visitor::new("shovel", "Hello Shovel, welcome home."),
	Visitor::new("yayo", "Hey Yayo take out the trash!"),
	Visitor::new("seagull", "Bird food was left outside from yesterday by the way."),
	Visitor::new("jizz", "Chinky Moment!"),
    ];
    let mut allow_them_in: bool = false;

    let known_visitor = visitor_list
	.iter()
	.find(|visitor| visitor.name == name);
    
    match known_visitor {
	Some(visitor) => visitor.greet_visitor(),
	None => println!("Hey {name}, you aren't on the list! You're going to have to leave.")
    }
}
