// Constant
// A constant ist defined using the `const` keyword.
// It lives through the entire lifetime of the program and has no fixed address in memory,
// as it is effectively "inlined" to each place it is used during compile.
// References to the same constant are not necessarily guaranteed to refer to the same memory address for this reason.
const MEANING_OF_LIFE:u8 = 42; // no fixed address

// global static variable
// a global variable that lives through the entire lifetime of the program
static X:i32 = 123;

// global static variable can be mutable, but it's not really memory safe, so
// it will give compile errors if you try to use it unless you wrap it in "unsafe"
static mut Z:i32 = 456;

fn main() {
    println!("Constant: {}", MEANING_OF_LIFE);

    println!("Global Static X: {}", X);

    // This will not work as it is unsafe
    // Z = 999;
    //println!("{}", Z);

    // to use the static mutable variable you need to wrap it in an unsafe block
    unsafe {
        println!("Global Static Z: {}", Z);
        // here we can change the value of the static mut
        Z = 999;
        println!("Global Static Z: {}", Z);
    }
}
