#![crate_name = "doc"]

/// A human struct
pub struct Person {
    /// The person name
    name: String,
}

impl Person {
    /// Returns a new person
    /// 
    /// # Arguments
    /// 
    /// * `name` - A string representing the person name
    /// 
    /// # Example
    /// 
    /// ```
    /// let Person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// Give a hello
    pub fn hello(& self) {
        println!("Hello {}!", self.name);
    }
}

fn main() {
    let john = Person::new("John");
    john.hello();
}