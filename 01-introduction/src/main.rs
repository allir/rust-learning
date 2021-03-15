// Instructions
// These instructions turn off warning from the compiler about various things.
// For example, we have unused imports, functions and variables in our program
// which the compiler would warn about without these instructions.
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

// Imports
// The `use` keyword is used to define imports. We never use the `std::mem` import, but
// we ignore the warning about it using the instructions above.
use std::mem;

// Functions
// The `fn` keyword defines a function
// This function is never used, but we ignore warning about it using the instructions
// above.
fn unused_func() {
    println!("Hahaha I'm not used!")
}

// Another function, this one prints out some text, we call this function from
// the application `main()` function.
fn used_func() {
    println!("Hello, from function!")
}

// Entry point to the application
// The `main()` function is a special function name and is called when the program
// is invoked.
fn main() {
    // Variables
    // The `let` keyword is used to define variables. This variable is not used, but
    // we ingore the warning with the instructions above.
    let haha = 15;

    // Another variable, which we do use.
    let world = "world!";
    
    // `println!` is a Macro that prints text out to the console. We use it here to print
    // out "Hello" and the value of `world` variable.
    println!("Hello, {}", world);

    // Invoke a function
    // Here we call our `used_func()` to be executed, which will print out some text to
    // the console.
    used_func();
}
