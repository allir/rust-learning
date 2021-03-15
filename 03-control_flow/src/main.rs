fn main() {
    if_statement();
    while_loop();
    for_loop();
    match_operation();
}

// IF Statement
// `if` statments evaluate an expression execute code (or not) base on the result
fn if_statement() {
    println!("# IF Statement");
    // We'll set a temperature variable and print out some text
    // based on the value of the variable using an if statement.
    let temp = 40;
    println!("Temperature is {}°C", temp);

    if temp > 30 {
        // If the temperature is above 30
        println!("It's really hot outside");
    } else if temp < 10 {
        // if it's below 10
        println!("That's really cold!")
    } else {
        // Any other value (10 - 30 in this case)
        println!("The temperature is OK")  
    }

    // We can also use an if statement during variable assignment,
    // the value of day will be set base on the value of temp.
    // The text within the curly-braces is assigned to the variable.
    // The returned value can be of any type, so it coudl also return a bool or integer etc.
    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}", day);

    // If can also be used within println!, if returns a value based on temp
    println!("It is {}",
        if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"OK"});

    // It's also possible to have nested if/else statements.
        println!("It is {}",
        if temp > 20 {
            if temp > 30 {"very hot"} else {"hot"}
        } else if temp < 10 {"cold"} else {"OK"}
    );
    print!("\n\n");
}

// While Loop
// A `while` loop will loop as long as an expression it is given is true
fn while_loop() {
    println!("# While Loop");

    // Let's create a variable named x and set it to 1
    let mut x = 1;
    // Loop while x is less than 1000
    while x < 1000 {
        // Multiply x by 2
        x *= 2;

        // The `continue` keyword can send skip the rest of the current loop and go back to the beginning.
        // If x is currently 64, we go back to the beginning of the loop,
        // So we don't print 64
        if x == 64 { continue; }

        // Print the current value of x
        println!("X = {}", x);
    } // Once the value of x is more than 1000 the loop ends

    // There is also a `loop` keyworld, which is essentially a "while true" loop.
    // It will run until it is manually "broken" using the `break` keyword
    let mut y = 1;
    loop {
        y *= 2;
        println!("y = {}", y);

        // If "y"'s value is 256 we break the loop 
        // 1<<8 is a bitshift by 8 bits. (10000000 in binary is 256)
        if y == 1<<8 { break; }
    }
    print!("\n\n");
}

// For Loop
// A `for` loop will loop for a certain amount of times, based on an expression
fn for_loop() {
    println!("# For Loop");
    // Loop 10 times 
    // 1..11 is inclusive of the first number, but exclusive of the second, so it's 1 - 10
    // z wil take the value currently being processed
    for z in 1..11 {
        // print the current value
        println!("z = {}", z);
    }

    // Loop 10 times
    for z in 1..11 {
        // But if the current value is 3 go back to the beginning of the loop
        if z == 3 { continue; }
        // Or if the current value is 8 we break the loop and continue with the program
        // In this case we won't actually get to process the loop 10 times.
        if z == 8 { break; }
        println!("z = {}", z);
    }

    // For each value between 30 - 40, which we enumerate,
    // take both the position in the enumeration and the value
    // and print it out.
    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
    print!("\n\n");
}

// Match
// The `match` keyword will check a variable and return a value or execute code
// based on the value.
// It's really useful where otherwise complicated if/else statements would be needed.
fn match_operation() {
    println!("# Match");
 
    // Set a variable to country code number.
    let country_code = 46;

    // Let's assign a value to the country variable base on the number.
    // Here the match will return a string value.
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "Unknown",
        _ => "Invalid"
    };

    println!("The country with code {} is {}", 
        country_code, country);

    // Match can also execute a codeblock. Here we will execute code based
    // on wether x is true or false.
    let x = false;
    match x {
        true => {
            println!("It's true...");
        },
        false => {
            println!("No way!");
        }
    };
}
