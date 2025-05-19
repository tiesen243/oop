pub trait Animal {
    fn name(&self) -> &str;
    fn age(&self) -> u32;

    fn show(&self) -> ();
    fn sound(&self) -> ();
    fn to_string(&self) -> String {
        format!("Animal[name={}, age={}]", self.name(), self.age())
    }
}
