fn main() {
    // Instead of moving the ownership of the memory by
    // passing the vector as an argument we can pass
    // a reference to a vector instead to "borrow"
    // the data.
    let print_vector = |x:&Vec<i32>| {
        println!("{:?}", x);
    };

    // Create a new vector and pass a reference to
    // it to the print_vector() closure
    let v = vec![1,2,3];
    print_vector(&v);
    // Now v has *not* moved so we can still call it
    println!("{:?}", v);

    // There some are rules on borrowing.
    // You can have multiple immutable references, but,
    // it is not possible to borrow a value as immutable and mutable 
    // at the same time.
    // It is also not possible to have multiple mutable references
    // at any given time.
    
    // A new i32 variable on the stack
    let mut a = 40;
    // b "borrows" via reference
    let b = &mut a;
    *b += 2;
    println!("{:?}", b);
    // c borrows via reference, unmutable
    let c = &a;
    println!("{:?}", c);
    // d also borrows via reference
    let d = &mut a;
    *d += 5;
    println!("{:?}", d);
    let e = &a;
    println!("{:?}", e);

    // The above code works and compiles, even though we
    // "break" those rules, as Rust figures out it can release
    // the references before using the next one, because
    // they are not used at a later time.
    
    // However, adding this line causes an error on "let d = &mut a"
    // as it's only possible to borrow "a" as mutable
    // once at a time.
    // It also breaks "let c = &a" as it's not possible to
    // borrow a variable as immutable when it's also borrowed as
    // as mutable.
    //println!("{:?}", b);

    
    // Let's create a mutable vector here
    let mut z = vec![3,2,1];
    
    // We borrow the vector here by referencing it
    // "for i in z" would result in a move!
    for i in &z {
        println!("i = {}", i);
        // If we try to push to the vector inside
        // the for loop we get an error, since the for
        // loop borrows z as immutable we cannot borrow it
        // as mutable at the same time!
        //z.push(5);
    }

    // Once the for loop has ended the immutable borrow of
    // z has been release so we can borrow it as mutable here
    z.push(5);
    println!("{:?}", z);
}
