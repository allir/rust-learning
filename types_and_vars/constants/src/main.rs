// constant
const MEANING_OF_LIFE:u8 = 42; // no fixed address

// global static variable
static X:i32 = 123;

// Memory Safety
// Global static variable can be mutable, but it will give compile errors if you try to use it unless you wrap it in "unsafe"
static mut Z:i32 = 456;

fn main() {
    println!("Constant: {}", MEANING_OF_LIFE);

    println!("Global Static X: {}", X);

    // This will not work as it is unsafe
    //Z = 999;
    //println!("{}", Z);

    // to use the static mutable variable you need to wrap it in an unsafe block
    unsafe {
        println!("Global Static Z: {}", Z);
        // you can even change it here
        Z = 999;
        println!("Global Static Z: {}", Z);
    }
}
