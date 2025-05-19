use crate::animal::Animal;

pub struct Dog {
    name: String,
    age: u32,
}

impl Animal for Dog {
    fn new(name: String, age: u32) -> Self {
        Dog { name, age }
    }

    fn show(&self) -> () {
        println!("Hi, I'm {}!, I'm {} years old!", self.name, self.age);
    }

    fn sound(&self) -> () {
        println!("Woof!");
    }

    fn to_string(&self) -> String {
        format!("Dog[name: {}, age: {}]", self.name, self.age)
    }
}
