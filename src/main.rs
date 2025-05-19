use oop::{Cat, Dog, Zoo};

fn main() {
    let mut zoo = Zoo::new(String::from("Ac nai"), vec![]);
    zoo.add(Box::new(Dog::new(String::from("Goldenglow"), 20)));
    zoo.add(Box::new(Cat::new(String::from("Pepe"), 19)));

    println!("{}", zoo.to_string());
    println!("Animals in the zoo:");
    zoo.show_animals();
}
