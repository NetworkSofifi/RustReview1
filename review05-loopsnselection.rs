//
// rustreview05-loops-selection    main.rs
//

use std::env; 
use std::process;

fn main() {
    let argvec : Vec<String> = env::args().collect();  // bundle cmd line args into vector

    if argvec.len() < 2 {           // ( ) not required around boolean condition
        println!("\nERROR:  expecting one or more command line arguments\n");
        process::exit(1);           // Exit without panic
    }                               // { } always required

    let mut s : String = String::new();
    let mut t : String = String::new();

    println!("s.is_empty() = { }", s.is_empty());
    println!("t.is_empty() = { }", t.is_empty());

    for k in 1..argvec.len() {
        println!("{ }  { }", k, argvec[k]);

        s.push_str(&argvec[k]);            // borrowed str slice from String value
    }

    let mut j : usize = 1;                 // Skip executable name
    while j < argvec.len() {
        t.push_str(argvec[j].as_str());    // as_str returns &str
        j += 1; 
    }

    println!("s = { }\t\tlen(s) = { }\tcap(s) = { }", s, s.len(), s.capacity());
    println!("t = { }\t\tlen(t) = { }\tcap(t) = { }", t, t.len(), t.capacity());

    println!("s.is_empty() = { }", s.is_empty());
    println!("t.is_empty() = { }", t.is_empty());
}
