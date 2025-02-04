//
// selectionfun  main.rs
// 

use std::env;
use std::str::FromStr;
 
fn main() {
    let mut numbers : Vec<u64> = Vec::new();
    
    let result1 : &str;
    let result2 : &str;

    if env::args().len() != 3 {
        println!("Usage error:  two integer arguments required");
        std::process::exit(1);
    } else {         // note this else is optional
        println!("processing now...");
    }

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    // Method 1
    if numbers[0] == numbers[1] {
        result1 = "==";
    } else if numbers[0] < numbers[1] {
        result1 = "<";
    } else {
        result1 = ">";
    }
    println!("\n");
    println!("result1: { } { } { }\n", numbers[0], result1, numbers[1]);

    // Method 2 - Nested IF - ELSE IF with tail expression
    result2 = if numbers[0] == numbers[1] {
                  "=="                            // Tail expression - No semicolon
              } else if numbers[0] < numbers[1] {
                  "<"
              } else {
                  ">"
              };                                  // Semicolon for assignment operation

    println!("result2: { } { } { }\n", numbers[0], result2, numbers[1]);
}
