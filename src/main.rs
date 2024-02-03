use std::io::stdin;
#[derive(Debug)]
#[allow(unused_variables)]

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting:  &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }
    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

enum VisitorAction {}

fn main() {
    println!("Enter name:");
    let username = user_input();
    println!("Hello, {:?}", username);
}

fn user_input() {
    let mut name = String::new();

    stdin()
        .read_line(&mut name)
        .expect("Failed to read");
    name.trim().to_lowercase();

    let mut visitor_list = vec![
        Visitor::new("Joseph", "Hello Joseph, enjoy the Tree"),
        Visitor::new("Kevin", "Hello Kevin, enjoy the Tree"),
        Visitor::new("Ian", "Hello Ian, enjoy the Tree"),
    ];
    let allow_in = false;

    let authorized = &visitor_list
        .iter()
        .find(|visitor| visitor.name == name);

    loop {
        println!("Enter name: (Leave empty and press Enter to quit program)");
        match authorized {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} not on visitor list", name);
                    &mut visitor_list.push(Visitor::new(&name, "New amigo"));
                }
            }
        }
    }

    if allow_in {
        println!("Welcome to the Tree {}!", name);
    } else {
        println!("Access denied");
    }
}