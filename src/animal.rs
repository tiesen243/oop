pub trait Animal {
    fn new(name: String, age: u32) -> Self;
    fn show(&self) -> ();
    fn sound(&self) -> ();
    fn to_string(&self) -> String;
}
