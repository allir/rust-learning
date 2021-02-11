#![allow(dead_code)]

fn if_statement() {
    let temp = 40;

    if temp > 30 {
        println!("really hot outside");
    } else if temp < 10 {
        println!("really cold!")
    } else {
      println!("temperature is OK")  
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}", day);

    println!("it is {}",
        if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"OK"});

    println!("it is {}",
        if temp > 20 {
            if temp > 30 {"very hot"} else {"hot"}
        } else if temp < 10 {"cold"} else {"ok"}
    );
}

fn while_loop() {
    let mut x = 1;

    while x < 1000 {
        x *= 2;

        // don't print 64
        if x == 64 { continue; }

        println!("X = {}", x);
    }

    let mut y = 1;
    // while true
    loop {
        y *= 2;
        println!("y = {}", y);

        if y == 1<<8 { break; }
    }

}

fn for_loop() {
    for z in 1..11 {
        println!("z = {}", z);
    }

    for z in 1..11 {
        if z == 3 { continue; }
        if z == 8 { break; }
        println!("z = {}", z);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

fn match_operation() {
    let country_code = 46;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "Unknown",
        _ => "Invalid"
    };

    println!("The country with code {} is {}", 
        country_code, country);

    let x = false;
    let s = match x {
        true => "yes",
        false => "no"
    };
    println!("{}", s)
}

fn main() {
    //if_statement();
    //while_loop();
    //for_loop();
    match_operation();
}
