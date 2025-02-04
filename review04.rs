//
//  rustreview04-strings     main.rs
//

use std::env;

fn main() {
    let mut argvec : Vec<String> = env::args().collect();

    // Output all command line strings
    println!("\n\n***** Unsorted *****");
    for k in 0..argvec.len() {
        println!("position { } => { }", k, argvec[k]);
    }

    // Compare file name to last cmd line argument
    assert_ne!(argvec[0], argvec[argvec.len()-1]);
    println!("\nASSERTION  { } != { }\n", argvec[0], argvec[argvec.len()-1]);


    argvec.remove(0);       // Remove executable file name


    // Sort remaining strings from command line
    argvec.sort();

    // Output sorting strings
    println!("\n*****  Sorted  *****");
    for k in 0..argvec.len() {
        println!("position { } => { }", k, argvec[k]);
    }
    println!("\n");

    // Compare first cmd line argument to last cmd line argument
    assert_ne!(argvec[0], argvec[argvec.len()-1]);
    println!("\nASSERTION  { } != { }\n", argvec[0], argvec[argvec.len()-1]);

}
