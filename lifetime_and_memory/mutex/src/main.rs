// Rc only works within a thread, so it's not safe to pass
// between threads.
//use std::rc::Rc;
// Atomic reference counted variables do though and work
// very similar, but they can not be mutated
// So we can 
use std::sync::{Arc,Mutex};
use std::thread;

struct Person {
    name: Arc<String>,
    state: Arc<Mutex<String>>
}

impl Person {
    fn new(name: Arc<String>) -> Person {
        let state = Arc::new(Mutex::new("bored".to_string()));
        Person { name: name, state: state }
    }

    fn greet(&self) {
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("excited");
        println!("Hi, my name is {} and I'm {}", self.name, state.as_str());
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
    let t = thread::spawn(move || {
        person.greet(); 
    });
    // After the thread has spawned 
    println!("Name = {}, name has {} strong pointers.", name, Arc::strong_count(&name));

    // We also borrow name here, but as immutable and without 
    // cloning, so it does not create a strong pointer
    println!("Name = {}", name);
    println!("Name = {}, name has {} strong pointers.", name, Arc::strong_count(&name));
    
    // Wait for the thread to finish and join
    t.join().unwrap(); 
    
    // After the thread returns we've release the pointer
    println!("Name = {}, name has {} strong pointers.", name, Arc::strong_count(&name));
}

fn main() {
    arc_demo();
}
