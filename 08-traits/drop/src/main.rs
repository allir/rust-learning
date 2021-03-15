#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]

struct Creature {
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enters the game", name);
        Creature { name: name.into() }
    }
}

// Implement the Drop trait for Creature
impl Drop for Creature {
    fn drop(&mut self) {
        // No need to do anything for memory management unless you're
        // doing something unsafe...
        println!("{} is dead!", self.name);
    }
}

// Drop is called when a variable is cleaned up so we can also explore
// when variables are cleaned up in regards to scope here...
fn main() {
    println!("Game starts...");
    // A "clever creature" will avoid death by moving from it's scope to here
    let clever: Creature;

    // This creature will be dropped at the end of main
    let giant_rat = Creature::new("Ratto");


    // start of scope
    {
    // This creature will be dropped manually using drop()
    let goblin = Creature::new("Jeff");
    
    // This creature will be dropped at the end of this scope
    let ogre = Creature::new("Shreek");
    
    // This creature will not be dropped at the end of this scope because we'll reassign it to clever
    let dragon = Creature::new("Red");

    println!("game proceeds...");

    // Reassign dragon to clever, causing it to move and so it will not be dropped at the end of this scope
    clever = dragon;
    // after this we can no longer use dragon, as it has been moved, the below line won't work...
    //println!("{}", dragon.name);

    // It's possible to drop a variable manually by calling drop()
    drop(goblin);

    println!("end of scope")
    }

    println!("end of main scope")
}
