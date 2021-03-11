#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

fn type_of<T>(_: &T) -> &'static str{
    std::any::type_name::<T>()
}

struct Point {
    x: f64,
    y: f64
}

fn origin() -> Point {
    Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap(){
    // stack allocated
    // Rust "stack allocates" by default
    let p1 = origin();

    // heap allocated
    // in Rust you can allocate memory on the heap with the Box<T> type
    let p2 = Box::new(origin());

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    // "unboxing" - reallocate from the heap to the stack
    // after this p2 can not be used again as the value has moved to p3
    let p3 = *p2;
    println!("p3 takes up {} bytes", mem::size_of_val(&p3));

    // Does not work anymore as p2 has been moved
    //println!("{}", p2.x)
}

fn main() {
    stack_and_heap();
}
