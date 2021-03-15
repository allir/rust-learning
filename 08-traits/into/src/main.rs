#![allow(unused_variables)]
#![allow(dead_code)]

struct Person {
    name: String
}

impl Person {
    // This funtion takes &str as an argument and converts it to a String
    //fn new(name: &str) -> Person {
    //    Person { name: name.to_string() }
    //}

    // but what if we already have a string? or another type, but it can be converted to string?
    // We can define the new function using a generic Type that has the Into<String> trait
    // using trait bound syntax:
    // fn new<S: Into<String>>(name: S) -> Person {
    // using a where clause:
    fn new<S>(name: S) -> Person
    where S: Into<String> {
        Person { name: name.into() }
    }
}

fn main() {
    // Create a person using an &str argument
    let john = Person::new("John");

    // Here we have a name that is already a String variable
    let name = "Jane".to_string();
    // We no longer need to convert to string to a ref after defining a new() that is using into()
    // let jane = Person::new(name.as_ref());
    // We pass in the String variable and the new() function will convert it using `name.into()`
    let jane = Person::new(name);
}
