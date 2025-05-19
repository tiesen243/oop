use crate::animal::Animal;

pub struct Dog {
    name: String,
    age: u32,
}

impl Dog {
    pub fn new(name: String, age: u32) -> Self {
        Dog { name, age }
    }
}

impl Animal for Dog {
    fn show(&self) -> () {
        println!("Hi, I'm {}!, I'm {} years old!", self.name, self.age);
    }

    fn sound(&self) -> () {
        println!("Woof!");
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn age(&self) -> u32 {
        self.age
    }

    fn to_string(&self) -> String {
        format!("Dog[name={}, age={}]", self.name, self.age)
    }
}
