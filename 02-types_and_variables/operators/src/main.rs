fn operators() {
    // Arithmetic
    // +, -, *, /
    let mut a = 2+3*4;
    a = a+1;
    a -= 2; // -=, +=, *=, /=, %=
    println!("{}", a);
    println!("remainder of {} / {} is {}", a, 3, (a%3));


    // Power of
    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed is {}", b, b_cubed);
    println!("{} to Pi is {}", b, b_to_pi);


    // Bitwise (for integers)
    // | = OR, & = AND, ^ = XOR, ! = NOR
    let c = 1 | 2;  // 01 OR 10 = 11 == 3_10
    println!("1 OR 2 = {}", c);


    // Shift / Bitshift
    // <<, >>
    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);


    // Logical (return a boolean)
    // >, <, <=, >=, ==
    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    println!("pi is less than 4 = {}", pi_less_4);
    
    let x = 5;
    let x_is_5 = x == 5;
    println!("x is 5 = {}", x_is_5);

}

fn main() {
    operators();
}
