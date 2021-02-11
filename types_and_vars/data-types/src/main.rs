#![allow(unused_variables)]
#![allow(unused_mut)]

// Imports
// We import the mem library so tha we can explore the size of the 
// variable types we're going to declare
use std::mem;

fn main() {
    // Integers
    // u8, u16, u32, u64, i8, i16, i32, i64
    // u stands for "unsigned integer" and i for "integer"
    // the number stands for the size of the integer in bits
    // An integer is a whole number.

    // Declaring variables;
    // Type declaration
    // the `let` keyword is used to create a new variable, using "type declaration"
    // we specifically declare the type of the variable using the syntax `let variable_name: type = value`.
    // By default variables are created as immutable, that is, it cannot be modified.
    let a: u8 = 123; // u = unsigned integer, 8 bits, immutable
    println!("a = {}", a);
    // Trying to change the value of `a` won't work.
    //a =  456; // This doesn't work as `a` is immutable
    println!("a = {}, and uses {} bytes", a, mem::size_of_val(&a));

    // the `mut` keyword is used to create a mutable variable, that is, it can be modified.
    let mut b: i8 = 0; // i = integer (signed), 8 bits, mutable
    println!("b = {} - before", b);
    b = 42; // chaning the value of `b` works as it is mutable
    println!("b = {} - after", b);
    println!("b = {}, and uses {} bytes", b, mem::size_of_val(&b));

    // Inferred type declaration
    // Declaring variables using "inferred" type uses the syntax `let variable_name = value` where
    // by omitting the type the type is "inferred", so rust will choose the most appropriate type.
    let c = 123456789; // i32, immutable
    println!("c = {}, and uses {} bytes", c, mem::size_of_val(&c));

    let mut d = 123456789; // i32, mutable
    println!("d = {} - before", d);
    println!("d = {}, and uses {} bytes", d, mem::size_of_val(&d));
    d = -1;
    println!("d = {} - after", d);
    println!("d = {}, and uses {} bytes", d, mem::size_of_val(&d));

    // usize, isize
    // the usize and isize are arhitecture specific variable types, which will give you an
    // integer of the bit size of your current operating system.
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, and uses {} bytes, {}-bit OS",
                z, size_of_z, size_of_z*8);


    // Characters
    // char
    // a "character" variable contain's a single utf-8 encoded character.
    let f: char = 'x';
    println!("{} is a char, and uses {} bytes", f, mem::size_of_val(&f));


    // Floating points
    // f32, f64
    // a "floating point" variable contain's a number which can be fractional.
    let i: f32 = 2.5; // f = floating point, 32 bit, immutable
    println!("i = {}, and uses {} bytes", i, mem::size_of_val(&i));

    let mut j: f64 = 2.5; // f = floating point, 64 bit, mutable
    println!("j = {}, and uses {} bytes", j, mem::size_of_val(&j));

    let k = 5.5; // inferred type floating point will be 64 bit...
    println!("k = {}, and uses {} bytes", k, mem::size_of_val(&k));


    // Boolean
    // bool
    // a boolean variable can be either true or false.
    let t: bool = true; // boolean, true/false
    println!("t = {}, and uses {} bytes", t, mem::size_of_val(&t));
}
