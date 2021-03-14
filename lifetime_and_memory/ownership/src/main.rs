#![allow(unused_variables)]

fn main() {
    // the variabl v "owns" the memory that is stored in the vector
    // v is stored on the stack as a pointer while the data is on the heap
    let v = vec![1,2,3];

    // only one variable can own the memory at any given time
    // when you assign v to v2 you are copying the pointer
    // but also the data ownership is "moved" to v2
    // and v is invalidated by Rust since it does not allow
    // more than one owner of the data
    let v2 = v;

    // So this won't work since v has moved
    //println!("{:?}", v);

    // The same thing happens here with v2 passing it into the closure
    // v2 get's copied to the closure's "v" and so it's moved and invalidated
    let foo = |v:Vec<i32>| ();
    foo(v2);

    // Again, this won't work as v2 has moved
    //println!("{:?}", v2);

    // For primitive types the data is copied and not moved
    // So assigning u to u2 copies the data instead of moving it.
    // u is an i32 and is on the stack
    let u = 1;
    let u2 = u;
    // So here we can print both u and u2
    println!("u = {} and u2 = {}", u, u2);

    // If we instead put an i32 on the heap, using Box
    // the situation changes again, p here is now a pointer to
    // an i32 so the data ownership is moved during the assignment
    // to p2
    let p = Box::new(1);
    let p2 = p;
    
    // Does not work since p has moved
    //println!("{:?}", p);

    println!("{:?}", p2);


    // An example of how the data ownership flows through a closure
    // Here the closure takes a vector as an argument and also
    // returns a vector.
    let print_vector = |x:Vec<i32>| -> Vec<i32> {
        println!("{:?}", x);
        x
    };
    // What happens here is that we create a new vector v
    // the ownership of the vector gets moved to x when
    // v is passed as an argument to print_vector() 
    // then the ownership of the data is moved to vv
    // since print_vector() returns a vector
    let v = vec![4,5,6];
    let vv = print_vector(v);
    println!("{:?}", vv);
    
    // As always this won't work as v has moved
    //println!("{:?}", v)
}
