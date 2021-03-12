#![allow(unused_mut)]

use std::ops::{Add,AddAssign,Neg};
use std::cmp::PartialEq;

// Some of these traits are also derivable, PartialEq and Eq for example
// #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[derive(Debug)]
struct Complex<T> {
    re: T,
    im: T
}

impl<T> Complex<T> { 
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> { re, im }
    }
}

// Implement Add for generic type where the type used also has the Add trait
impl<T> Add for Complex<T>
where T: Add<Output = T> {
    type Output = Complex<T>;

    // self + rhs
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

// Implement AddAssign for generic type where the type used also has the AddAssign trait
impl<T> AddAssign for Complex<T>
where T: AddAssign<T> {
    // self + rhs
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

// Implement Neg for generic type where the type used also has the Neg trait
impl<T> Neg for Complex<T>
where T: Neg<Output=T> {
    type Output = Complex<T>;
    // self + rhs
    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re, 
            im: -self.im
        }
    }
}

// Equality
// There are two actually, partial equality and full equality
// full equaility is not possible for floating points, as it can return a NaN (not a number)
// and NaN == NaN returns false

impl<T> PartialEq for Complex<T>
where T: PartialEq {
    fn eq(&self, rhs: &Self) -> bool {
        self.re == rhs.re && self.im == rhs.im
    }
}

// To implement full equality it re-uses the PartialEq implementation so you don't need to implement it specifically
impl<T: Eq> Eq for Complex<T>
where T: Eq {}

fn main() {
    let mut a = Complex::new(1.0, 2.3);
    let mut b = Complex::new(3.1, 4.5);

    println!("{:?}", a);
    println!("{:?}", b);

    // This does not work without implementing the Add trait
    println!("{:?}", a + b);

    // Since we implement add as a generic we use different types
    // Note: the types being added need to be of the same type though
    let mut c = Complex::new(1, 2);
    let mut d = Complex::new(3, 4);
    println!("{:?}", c + d);

    // We need to reassign c & d because they have moved in the add operation above
    c = Complex::new(1, 2);
    d = Complex::new(3, 4);
    // This does not work without implementing AddAssign trait
    c += d;
    println!("{:?}", c);

    // This does not work without implementing the Neg trait
    println!("{:?}", -c);


    c = Complex::new(1, 2);
    d = Complex::new(1, 2);
    // This does not work without implementing the PartialEq trait
    println!("{}", c == d);

}
