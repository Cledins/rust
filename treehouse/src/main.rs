use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction, 
    age: i8,
}

#[derive(Debug)]
enum VisitorAction {
    Accept, 
    AcceptWithNote { note: String }, 
    Refuse, 
    Probation,
}

impl Visitor {
    fn new (name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action, 
            age
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("welcome {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("welcome {}", self.name);
                println!("{}", note);
                if self.age <21 {
                    println!("do not serve alcohol to {}", self.name);
                }   
            },
            VisitorAction::Probation => println!("{} is on probation", self.name),
            VisitorAction::Refuse => println!("{} is not allowed", self.name),
        }
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read name");
    your_name
        .trim()
        .to_lowercase()
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("bert", VisitorAction::Accept, 45), 
        Visitor::new("steve", VisitorAction::AcceptWithNote{
            note: String::from("milk in fridge")
        }, 15),
        Visitor::new("Fred", VisitorAction::Refuse, 30)
    ];

    loop {
        println!("Hello, what's your name ? (Leave blank and ENTER to quit)");
        let name = what_is_your_name();

        let known_visitor = visitor_list
            .iter()
            .find(|visitor| visitor.name == name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                }
                else {
                    println!("{} is new", name);
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
                }

            }
        }
    }

    println!("The final list of visitors :");
    println!("{:#?}", visitor_list);
}

