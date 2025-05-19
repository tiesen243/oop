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
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        self.name.hash(&mut hasher);
        self.age.hash(&mut hasher);
        let hash = hasher.finish();

        format!("Dog@{:x}[name={},age={}]", hash, self.name, self.age)
    }
}
