// Rc only works within a thread, so it's not safe to pass
// between threads.
//use std::rc::Rc;
// Atomic reference counted variables do though and work
// very similar
use std::sync::Arc;
use std::thread;

struct Person {
    name: Arc<String>
}

impl Person {
    fn new(name: Arc<String>) -> Person {
        Person { name: name }
    }

    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}
fn arc_demo() {
    let name = Arc::new("John".to_string());
    println!("Name = {}, name has {} strong pointers.", name, Arc::strong_count(&name));
    // We need to use the clone() funtion so name does not
    // get moved at this point.
    let person = Person::new(name.clone());
    println!("Name = {}, name has {} strong pointers.", name, Arc::strong_count(&name));

    // Spawn a new thread and call the greeting from there
    // Without using arc this would cause an error.
    let t = std::thread::spawn(move || {
        person.greet(); 
    });
    // After the thread has spawned 
    println!("Name = {}, name has {} strong pointers.", name, Arc::strong_count(&name));

    // We also borrow name here, its does not create a strong pointer as we're not clone()-ing
    println!("Name = {}", name);
    println!("Name = {}, name has {} strong pointers.", name, Arc::strong_count(&name));
    
    // Wait for the thread to finish and join
    t.join().unwrap(); 
    
    println!("Name = {}, name has {} strong pointers.", name, Arc::strong_count(&name));
}

fn main() {
    arc_demo();
}
