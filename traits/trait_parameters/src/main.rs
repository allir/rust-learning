use std::fmt::Debug;

// The struct will derive the Debug trait from std::fmt::Debug
// so we don't need to implement it ourselves (but we can ofc)
#[derive(Debug)]
struct Circle {
    radius: f64
}

#[derive(Debug)]
struct Square {
    side: f64
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// There are 3 ways to define trait parameters
// 1. impl trait syntax, decorate the argument with traits
// fn print_info(shape: impl Shape + Debug) {
// 2. trait bound syntax, this is easier when you have multiple arguments of the same type
// fn print_info<T: Shape + Debug>(shape: T)
// 3. trait bound with where clause, easier when you have multiple types
fn print_info<T>(shape: T) 
where T: Shape + Debug {
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}

fn main() {
    let c = Circle { radius: 2.0 };
    print_info(c)
}
