// Create/Define an Animal trait
trait Animal {
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    
    // the talk function has a default implementation
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

struct Mouse {
    cat_name: &'static str
}

// Implement the animal trait for Human
impl Animal for Human {
    fn create(name: &'static str) -> Human {
        Human{name: name}
    }
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat{cat_name: name}
    }
    
    fn name(&self) -> &'static str {
        self.cat_name
    }

    fn talk(&self) {
        println!("{} says meow", self.name())
    }
}

impl Animal for Mouse {
    fn create(name: &'static str) -> Mouse {
        Mouse{cat_name: name}
    }
    
    fn name(&self) -> &'static str {
        self.cat_name
    }

    // The talks function has a default implementation on the trait
    // so if you don't define one it will be inherited...
    //fn talk(&self) {
    //    println!("{} sqeaks", self.name())
    //}
}


trait Summable<T> {
    fn sum(&self) -> T;
}

// Implement traits on existing types/objects
impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result:i32 = 0;
        for x in self { result += *x; };
        return result;
    }
}

fn traits() {
    //let h = Human{ name: "John" };
    let h = Human::create("John");
    // You can also call the Animal trait by specifying the variable type
    // let h:Human = Animal::create("John");
    h.talk();

    //let c = Cat{ cat_name: "Kitty" };
    //let c = Cat::create("Kitty");
    let c:Cat = Animal::create("Kitty");
    c.talk();

    let m:Mouse = Animal::create("Fievel");
    m.talk();


    // a Vec<i32> does not have the sum() trait, but we implement it above!
    let a = vec![1, 2, 3];
    println!("sum of a = {}", a.sum())
}



fn main() {
    traits();
}
