fn how_many(x:i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "dozen",
        _ if ( x > 13) => "many",
        _ => "a few",
    }
}

pub fn pattern_matching(){
    for x in 0..25 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let point = (3,4);
    match point {
        (0,0) => println!("origin"),
        (0,y) => println!("x axis, y = {}", y ),
        (x,0) => println!("y axis, x = {}", x),
         // you can pass the data as a reference and even as mutable with the ref & mut keywords
        // (ref mut x,0) => println!("y axis, x = {}", x),
        (x,y) => println!("At coordinates: {},{}",x,y)
    }
}
