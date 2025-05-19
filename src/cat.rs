use crate::animal::Animal;

pub struct Cat {
    name: String,
    age: u32,
}

impl Cat {
    pub fn new(name: String, age: u32) -> Self {
        Cat { name, age }
    }
}

impl Animal for Cat {
    fn show(&self) -> () {
        println!("Hi, I'm {}!, I'm {} years old!", self.name, self.age);
    }

    fn sound(&self) -> () {
        println!("Meow!");
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn age(&self) -> u32 {
        self.age
    }

    fn to_string(&self) -> String {
        format!("Cat[name={}, age={}]", self.name, self.age)
    }
}
