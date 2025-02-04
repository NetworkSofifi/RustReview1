//
// rustreview02-modules   main.rs
//

mod samplelib;
use samplelib::somefunc;          // Similar to a C++ scope resolution

fn main() {
    println!("***********");

    let j = 2;                    // Type inference used here
    let mut k : u32 = 4;          // k must be mutable so that it can be incremented
    println!("k = { }", k);
    somefunc(j, k);

    println!("***********");

    k += 1;                       // Equivalent to k = k + 1;
    println!("k = { }", k);
    somefunc(j, k);

    println!("***********");
}
