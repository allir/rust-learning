use std::io::{stdin, stdout};
use std::io::Write;  /* for flushing stdout */

fn main() {
    println!("Hello, Mars!");

    print!("Enter your weight (kg): ");
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let earth_weight = input.trim().parse().unwrap();
    let mars_weight = calculate_mars_weight(earth_weight);
    println!("Weight on Mars: {} kg", mars_weight);
}

fn calculate_mars_weight(earth_weight: f32) -> f32 {
    (earth_weight / 9.81) * 3.711
}
