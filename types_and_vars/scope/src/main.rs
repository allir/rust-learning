fn scope_and_shadowing() {
    // a is delcared and exists inside the scope_and_shadowing function
    let a = 123;
    println!("in function: a = {}", a);

    // variables declared inside code blocks exist inside that code block only
    {
        // a will be accessible within this codeblock as it is part of the function where it is declared
        println!("in codeblock: a = {}", a);

        // a can be redeclared inside this block scope and will "shadow" the a declared in the function scope
        let a = 789;
        println!("in codeblock: a (shadow) = {}", a);

        let b = 456;
        println!("in codeblock: b = {}", b);
    }

    // a here is back to the value set in the function before the code block. so the "shadowed" a is gone
    println!("in function: a = {}", a)
    
    // This won't work, b does not exist outside the above code block.
    //println!("function: b = {}", b);


}

fn main() {
    scope_and_shadowing()

}
