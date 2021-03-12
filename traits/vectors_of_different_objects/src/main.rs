trait Animal {
    // fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    cat_name: &'static str
}

impl Animal for Human {
    // fn create(name: &'static str) -> Human {
    //     Human{name: name}
    // }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

impl Animal for Cat {
    // fn create(name: &'static str) -> Cat {
    //     Cat{cat_name: name}
    // }
    
    fn name(&self) -> &'static str {
        self.cat_name
    }

    fn talk(&self) {
        println!("{} says meow", self.name())
    }
}

// Implement an enum so that we can add different types to an Vec
enum Creature {
    Human(Human),
    Cat(Cat)
}

fn main() {
    let mut creatures = Vec::new();

    // You can not have a Vec of different types so this won't work
    //creatures.push(Human{name:"John"});
    //creatures.push(Cat{cat_name:"Fluffey"});

    // We can abstract the Creatures using an enum
    creatures.push(Creature::Human(
        Human { name: "John" }
    ));
    creatures.push(Creature::Cat(
        Cat { cat_name: "Fluffey" }
    ));

    for c in creatures {
        match c {
            Creature::Human(h) => h.talk(),
            Creature::Cat(c) => c.talk()
        }
    }

    // Or we can type the vector with the Animal trait
    // The Vec also needs to know the size of the stored element at compile time
    // so we have to Box the Type, that way we get a fixed size pointer while the Animal
    // variable is placed on the heap
    let mut animals:Vec<Box<dyn Animal>> = Vec::new();
    animals.push(
        Box::new(Human{name:"John"}));
    animals.push(
        Box::new(Cat{cat_name:"Fluffey"}));

        for a in animals.iter() {
            a.talk()
        }
}
