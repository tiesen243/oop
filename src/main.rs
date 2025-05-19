mod animal;
mod cat;
mod dog;

use crate::animal::Animal;
use crate::cat::Cat;
use crate::dog::Dog;

fn main() {
    let gg = Dog::new(String::from("Goldenglow"), 5);
    gg.show();
    gg.sound();
    println!("{}", gg.to_string());

    let pepe = Cat::new(String::from("Pepe"), 19);
    pepe.show();
    pepe.sound();
    println!("{}", pepe.to_string());
}
