//
// enumfun   main.rs
//

use std::env;
use std::str::FromStr;

enum Ordering {
    LessThan,
    EqualTo,
    GreaterThan
}

fn compare( m : u32, n : u32) -> Ordering {
    if m < n {
        Ordering::LessThan
    } else if m == n {
        Ordering::EqualTo
    } else {
        Ordering::GreaterThan
    }
} 

fn ratio( m : u32, n : u32) -> Option<u32> {
    if n == 0 {
        None
    } else {
        Some(m / n)
    }
}

fn main() {

    let argvec : Vec<String> = env::args().collect();  // bundle cmd line args into vector

    if argvec.len() != 3 {                             // Exit gracefully
        println!("Usage error: expecting two integer arguments");
        return;
    }    

    // Cmd line argument supplies limit 
    let m : u32 = u32::from_str(&argvec[1]).expect("parse error"); 
    let n : u32 = u32::from_str(&argvec[2]).expect("parse error"); 

    print!("m is ");
    match m {
        0 => println!("zero"),
        1 | 3 | 5 | 7 | 9 | 11 | 13 | 15 | 17 | 19 => println!("odd"),
        2 | 4 | 6 | 8 | 10 | 12 | 14 | 16 | 18 => println!("even"),
        20..=29 => println!("twenties"),
        _ => println!("huh?")
    }

    print!("n is ");
    match n {
        0 => println!("zero"),
        1 | 3 | 5 | 7 | 9 | 11 | 13 | 15 | 17 | 19 => println!("odd"),
        2 | 4 | 6 | 8 | 10 | 12 | 14 | 16 | 18 => println!("even"),
        20..=29 => println!("twenties"),
        _ => println!("huh?")
    }

    let result : &str = match compare(m, n) {
        Ordering::LessThan => "<",
        Ordering::EqualTo => "==",
        Ordering::GreaterThan => ">"
    };

    println!("m = { }   { }   n = { }    ", m, result, n);  

    match ratio(m, n) {                            // like a switch from C/C++
        None => println!("cannot divide by zero"),
        Some(x) => println!("m / n = { }", x)      // x is placeholder
    }
}
