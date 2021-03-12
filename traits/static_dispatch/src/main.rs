trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        format!("string: {}", *self)
    }
}

// Monomorphisation
// At compile time concrete implementations of this function will be created
// for the types that you request, eg. print_it(z: String) and print_it(z: i32)
fn print_it<T: Printable>(z: T){
    println!("{}", z.format());
}

fn main() {
    let a = 123; 
    let b = "hello".to_string();

    //println!("{}", a.format());
    //println!("{}", b.format());

    // Static dispatch
    // "which" print_it function is being called, print_it(z: String) or print_it(z: i32),
    // is made at compile time. So these calls are replaced by the concrete implementations 
    print_it(a);
    print_it(b);

}
