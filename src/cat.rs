use crate::animal::Animal;

pub struct Cat {
    name: String,
    age: u32,
}

impl Animal for Cat {
    fn new(name: String, age: u32) -> Self {
        Cat { name, age }
    }

    fn show(&self) -> () {
        println!("Hi, I'm {}!, I'm {} years old!", self.name, self.age);
    }

    fn sound(&self) -> () {
        println!("Meow!");
    }

    fn to_string(&self) -> String {
        format!("Cat {{ name: {}, age: {} }}", self.name, self.age)
    }
}
