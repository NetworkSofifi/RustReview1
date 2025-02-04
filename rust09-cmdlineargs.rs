//
// cmdlineargfun  main.rs
//

use std::env;
use std::str::FromStr;

fn main() {

    // Method 1 - collect all cmd line args and place in a String vector
    println!("\n**** Method 1 ****");
    let argvec : Vec<String> = env::args().collect();
    println!("Number of cmd line args = { }", argvec.len());
    for k in 0..argvec.len() {
        println!("position { } => { }   length = { }", k, argvec[k], argvec[k].len());
    }

    // Method 2 - use an iterator to step through the cmd line args
    println!("\n**** Method 2 ****");
    for j in env::args() {
        println!("{ }   length = { }", j, j.len());
    }

    // Skip the path arg and convert remaining args to u32
    println!("\nSkipping the executable name");
    for a in env::args().skip(1) {
        println!("{ }   length = { }   equivalent int = { }", 
                   a, a.len(), u32::from_str(&a).expect("parse error"));
    }
}
