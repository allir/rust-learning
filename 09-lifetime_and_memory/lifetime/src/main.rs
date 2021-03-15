struct Cat {
    // This wont' work as it expects a lifetime for the
    // reference as a guarantee that the reference will not
    // cease to exist before the struct.
    //name: &str
    
    // ' designates a lifetime
    // 'static is a special lifetime keyword that indicates 
    // that the data pointed to by the reference lives for 
    // the entire lifetime of the running program.
    // name: &'static str
    name: &'static str
}

impl Cat {
    fn talk(&self) {
        println!("{} says meow", self.name);
    }
}

struct Person {
    name: String
}

impl Person {
    fn talk(&self) {
        println!("Hi my name is {}", self.name)
    }
    
    fn  get_ref_name(&self) -> &String {
        &self.name
    }
}

// Company has a reference to a Person
// We can specify a lifetime parameter for the struct and the
// and Person to give a guarantee that they will have the same lifetime.
// "a" is or own name we give to the lifetime here, it's just an identifier
struct Company<'a> {
    name: String,
    ceo: &'a Person
}

// Any implementations for a struct with a lifetime parameter will
// also need to specify the lifetime.
impl<'a> Company<'a> {
    fn print_info(&self) {
        println!("{}'s CEO is {}", self.name, self.ceo.name);
    }
}

fn main() {
    let name = "Kitteh";
    let cat = Cat { name: name };
    cat.talk();

    let person = Person { name: "Elon".to_string() };
    person.talk();

    let company = Company { name: String::from("Nicolai Cars"), ceo: &person };
    company.print_info();

    // Here is an example of where we get an error because
    // an object would stop existing but still have a reference to it
    // We create a varable z that is a String reference
    let z: &String;
    {
        // Inside a new scop we create a new Person p
        let p = Person { name: String::from("John") };
        // Assigning a reference to p's name here, but after
        // p's scope ends we try to use z causes a lifetime
        // error, p "does not live long enough" to use a refernce
        // to it's name outside of p's scope.
        z = p.get_ref_name();
    }
    // Borrowing z here is what causes the above error
    println!("{}", z)
    
}
