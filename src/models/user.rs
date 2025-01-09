use crate::traits::display::Display;

#[derive(Clone)]
pub struct User {
    pub name: String,
    pub age: u32,
}

impl User {
    pub fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }

    pub fn greet(&self) -> String {
        format!("Hello, my name is {} and I am {} years old.", self.name, self.age)
    }
}

impl Display for User {
    fn info(&self) -> String {
        format!("User: {}, Age: {}", self.name, self.age)
    }
}