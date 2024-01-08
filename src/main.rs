use std::io::stdin;

fn main() {
    println!("Enter name:");
    let username = user_input();
    println!("Hello, {}", username);
}

fn user_input() -> String {
    let mut name = String::new();

    stdin()
        .read_line(&mut name)
        .expect("Failed to read");
    name
        .trim()
        .to_lowercase();

    let visitor_list = ["Wambua", "Mwangi", "Wandeto"];
    let mut allow_in = false;

    for visitor in &visitor_list {
        if visitor == &name {
            allow_in = true;
        }
    }

    if allow_in {
        println!("Welcome to the Tree {}!", name);
    } else {
        println!("Access denied");
    }
}