#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::mem;

fn main() {
    // intgers; u8, u16, u32, u64, i8, i16, i32, i64

    // type declaration
    let a: u8 = 123; // u = unsigned integer, 8 bits, immutable
    println!("a = {}", a); // immutable
    // a =  456; // doesn't work "a" is immutable

    let mut b: i8 = 0; // i = integer (signed), 8 bits, mutable
    println!("b = {} - before", b);
    b = 42;
    println!("b = {} - after", b);

    // inferred type
    let c = 123456789; // i32, immutable
    println!("c = {}, and uses {} bytes", c, mem::size_of_val(&c));

    let mut d = 123456789; // i32, mutable
    println!("d = {}, and uses {} bytes", d, mem::size_of_val(&d));
    d = -1;
    println!("d = {}, and uses {} bytes", d, mem::size_of_val(&d));


    // usize, isize
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, and uses {} bytes, {}-bit OS",
                z, size_of_z, size_of_z*8);


    // char
    let f: char = 'x';
    println!("{} is a char, and uses {} bytes", f, mem::size_of_val(&f));


    // floating-points; f32, f64
    let i: f32 = 2.5; // f = floating point, 32 bit, immutable
    println!("i = {}, and uses {} bytes", i, mem::size_of_val(&i));

    let mut j: f64 = 2.5; // f = floating point, 64 bit, mutable
    println!("j = {}, and uses {} bytes", j, mem::size_of_val(&j));

    let k = 5.5; // inferred type floating point will be 64 bit...
    println!("k = {}, and uses {} bytes", k, mem::size_of_val(&k));


    // bool
    let t: bool = true; // boolean, true/false
    println!("t = {}, and uses {} bytes", t, mem::size_of_val(&t));
}
