#![allow(unused_imports)]

use std::mem;

enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple
    CmykColor{cyan: u8, magenta: u8, yellow: u8, black: u8} // struct
}

fn enums() {
    let c:Color = Color::CmykColor{cyan: 0, magenta: 0,yellow: 0, black: 255};

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0,0,0) 
        |  Color::CmykColor{cyan: _, magenta: _, yellow: _, black: 255} => println!("black"),
        Color::RgbColor(r,g,b) => println!("rgb: {},{},{}", r,g,b),
        Color::CmykColor{cyan: c, magenta: m, yellow: y, black: k} => println!("cmyk: {},{},{},{}", c,m,y,k),
        _ => ()
    }
}

fn main() {
    enums()
}
