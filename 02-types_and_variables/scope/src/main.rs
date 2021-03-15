fn scope_and_shadowing() {
    // main() function's "a" does not exist here, as we don't pass in any arguments to this function.
    // So we can declare a inside the function and exists while the function executes.
    let a = 123;
    println!("in function: a = {}", a);

    // A new code block or scope can be defined by curly braces.
    // Variables declared inside code blocks exist inside that code block only.
    {
        // "a" will be accessible within this codeblock as it is part of the function where it is declared.
        println!("in codeblock: a = {}", a);

        // "a" can be redeclared inside this codeblock, as it's a new scope and will "shadow" the a declared in the function scope,
        // that is, this code block will now use this new "a" over the other one.
        let a = 789;
        println!("in codeblock: a (shadow) = {}", a);

        // Let's also define a new variable "b" within the code-block
        let b = 456;
        println!("in codeblock: b = {}", b);
    
    } // codeblock "a" and "b" are dropped

    // a is now back to the value set in the function before the code block. so the "shadowed a" is gone
    println!("in function: a = {}", a)
    
    // This won't work, b does not exist outside the above code block.
    //println!("function: b = {}", b);

} // function "a" is dropped

fn main() {
    // a variable "a" is defined in the main function, and exists while the function executes
    // this variable does not exist within functions that main calls, unless we would pass it in as an argument
    let a = "scope demo";
    println!("in main: a = {}", a);
    
    scope_and_shadowing();

    // "a" in main is still the same value as before as it was never passed to the function
    // and has just been waiting in the main() function.
    println!("in main: a = {}", a);

} // main scope "a" is dropped
