#![allow(unused_variables)]

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
    let d:Point<f32> = Point { x: 1.2, y: 3.4};

    // two different types
    let e:AnotherPoint<u16,i32> = AnotherPoint {x: 5, y: 5};

    // As Line only takes one type, both start and end must be the same type of Point...
    // b and d are both Point<f32>
    let my_line = Line { start: b, end: d};
    // Does not work:
    // let myLine = Line { start: a, end: b};


}

fn main() {
    generics();
}
