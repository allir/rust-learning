#![allow(unused_variables)]

fn type_of<T>(_: &T) -> &'static str{
    std::any::type_name::<T>()
}

struct Point<T> {
    x: T,
    y: T
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>
}

struct AnotherPoint<T, V> {
    x: T,
    y: V
}

fn generics(){
    // type deduction
    let a = Point { x: 0, y: 0 };
    let b = Point { x: 1.2, y: 3.4};
    
    // explicit types
    let c:Point<u64> = Point { x: 0, y: 0 };
    let d:Point<f32> = Point { x: 2.5, y: 8.5};

    // AnotherPoint can take the same or different types
    // same type via type deduction
    let e = AnotherPoint {x: 5, y: 5};
    // two different types
    let f:AnotherPoint<u16,i32> = AnotherPoint {x: 5, y: 5};

    // As Line only takes one type, both start and end must be the same type of Point...
    // b and d are both Point<f32>
    let my_line = Line { start: b, end: d};
    // Does not work! a and b are Point<i32> and Point<f64>
    // let myLine = Line { start: a, end: b};

    println!("My line goes from ({},{}) to ({},{})\n", my_line.start.x, my_line.start.y, my_line.end.x, my_line.end.y);

    println!("AnotherPoint can take different types for x & y:");
    println!("e = AnotherPoint<{}, {}>", type_of(&e.x), type_of(&e.y));
    println!("f = AnotherPoint<{}, {}>", type_of(&f.x), type_of(&f.y));

}

fn main() {
    generics();
}
