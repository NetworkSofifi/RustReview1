//
// mathproject  main.rs
//

fn square(n : i32) -> i32 {
    n*n     // Equivalent to return n*n;
}

fn main() {
    let k : i32 = 2;
    println!("The square of { } is { }", k, square(k));
    
    let mut k : i32 = 3;
    println!("The square of { } is { }", k, square(k));

    k = 4;
    println!("The square of { } is { }", k, square(k));

    for k in ["cat", "dog", "goat"] {  
        println!("animal = { }", k);
    }

    k = 5;
    println!("The square of { } is { }", k, square(k));
}
