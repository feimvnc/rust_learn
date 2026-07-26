// use r03_palindrome::Palindrome;

mod my_crate; // Declares the module
use my_crate::Palindrome;

fn main() {
    let s = String::from("racecar");
    println!("Is palindrome: {}", s.is_palindrome());
}
