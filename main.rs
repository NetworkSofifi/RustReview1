//
// reviewreview01   main.rs
//

fn main() {
    
    let n : u32 = 5;   // Explicit type specification

    let m = 10_i32;    // Context for type inference

    let x;             // Split declaration and initialization
    x = 15;            // Type inferred by context
    
    let mut y : i32 = 20;             // Error1 - no type specification - add : i32
                       // Error2 - uninitialized variable - add = 20 

    println!("n = { } and m = { }", n, m);    // similar to C printf

    print!("(x,y) = ");        // Does not add newline
    println!("({x}, {y})");    // Adds newline
}
