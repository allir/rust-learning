use std::mem;

// this function exits everywhere
fn say_hello() { println!("hello"); }

fn closures(){
    //say_hello();

    // create a variable that referst to the function
    let sh = say_hello;
    sh();

    // create function using a closure, it only exists inside this function
    let plus_one = |x:i32| -> i32 { x +1 };
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    // a little more complicated closure
    // declare two as a mutable variable
    let mut two = 2;
    {
        let plus_two = |x| {
            let mut z = x;
            z += two;
            z
        };
        println!("{} + 2 = {}", 3, plus_two(3));
    }    
    // to make sure I can borrow two here the plus_two needs
    // to be wrapped in a scope?
    let borrowed_two = &mut two;
    println!("{}", borrowed_two);

    // T: by Value
    // &T: by Reference
    // &mut T: mutable Reference
    let plus_three = |x: &mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}", f)

}

fn main() {
    //say_hello();
    closures();
}
