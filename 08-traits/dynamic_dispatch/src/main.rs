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

fn print_it(z: &dyn Printable) {
    // Dynamic Dispatch
    // There is only this single implementation of print_it now.
    // The function looks at which type z is to make a decision on which format() to call depending on the type.
    // This happens at runtime but not at compile time.
    println!("{}", z.format());
}

struct Circle { radius: f64 }
struct Square { side: f64 }

trait Shape { fn area(&self) -> f64; }

impl Shape for Circle { fn area(&self) -> f64 { self.radius * self.radius * std::f64::consts::PI }}
impl Shape for Square { fn area(&self) -> f64 { self.side * self.side }}

fn main() {
    let a = 123; 
    let b = "hello".to_string();

    //println!("{}", a.format());
    //println!("{}", b.format());

    // Now we pass a reference to print_it
    print_it(&a);
    print_it(&b);


    // Another case of where dynamic dispatch is used,
    // In this case we have an array of more than one type, so when we call "shape.area()"
    // in our for loop it will need to look at the type of the shape and call the appropriate
    // area() function at runtime.
    let shapes:[&dyn Shape; 4] = [
        &Circle{radius: 1.0},
        &Square{side: 2.0},
        &Circle{radius: 2.0},
        &Square{side: 4.0}
    ];

    for (i, shape) in shapes.iter().enumerate() {
        println!("Shape #{} has area {}", i, shape.area());
    }

}
