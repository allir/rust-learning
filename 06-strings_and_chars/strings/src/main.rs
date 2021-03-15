fn strings() {
    // utf-8
    
    // String Slice
    // &str = string slice
    // static = the string is statically built into the binary
    let s:&'static str = "Hello there!";
    println!("{}",s);

    //for c in s.chars().rev() {
    for c in s.chars() {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("The first character is: {}", first_char)
    }


    // String
    // heap allocated
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }

    println!("{}", letters);


    // Conversion
    // &str <> String
    let u:&str = &letters;
    println!("{}", u);

    // Concatenation
    // String + &str
    let z = letters + "1,2,3,4,5";
    println!("{}", z);

    // String + String (really String + &str)
    let s1 = String::from("A string");
    let s2 = "Another string".to_string();
    let mut y = s1 + " and " + &s2;
    println!("{}", y);

    // Strings have methods
    y.remove(0); // Remove the first char
    y.remove(0); // Remove the first char
    y.push_str("!!!"); // Add to the end of the string
    println!("{}",y.replace("string", "REPLACED"))

}

fn main() {
    strings();
}
