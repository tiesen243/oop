use crate::animal::Animal;

pub struct Zoo {
    name: String,
    animals: Vec<Box<dyn Animal>>,
}

impl Zoo {
    pub fn new(name: String, animals: Vec<Box<dyn Animal>>) -> Self {
        Self { name, animals }
    }

    pub fn add(&mut self, animal: Box<dyn Animal>) {
        self.animals.push(animal);
    }

    pub fn show_animals(&self) {
        for animal in &self.animals {
            println!("{}", animal.to_string());
        }
    }

    pub fn to_string(&self) -> String {
        format!("Zoo[name={}, animals: {}]", self.name, self.animals.len())
    }
}
