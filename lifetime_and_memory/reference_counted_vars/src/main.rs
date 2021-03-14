use std::rc::Rc;

struct Person {
    name: Rc<String>
}

impl Person {
    fn new(name: Rc<String>) -> Person {
        Person { name: name }
    }

    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}
fn rc_demo() {
    let name = Rc::new("John".to_string());
    println!("Name = {}, name has {} strong pointers.", name, Rc::strong_count(&name));
    
    // Let's start a new scope hre
    {
        // To prevent name from being moved here we can
        // use refrene-counting's clone() function to pass
        // a strong pointer to name.
        let person = Person::new(name.clone());
        println!("Name = {}, name has {} strong pointers.", name, Rc::strong_count(&name));
        person.greet();
        // When this scope ends, person gets cleaned up 
        // and also the strong pointer/clone to name
    }

    // Because name was not moved, we can still use it here and our 
    // number of strong pointers should be back down to 1.
    println!("Name = {}, name has {} strong pointers.", name, Rc::strong_count(&name));
}

fn main() {
    rc_demo();
}
