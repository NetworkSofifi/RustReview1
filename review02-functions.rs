//
// reviewreview02-functions   main.rs
//

fn voidfunc() {                       // No return type listed
    println!("void function here");   // ! indicates a macro
}

fn mystery() -> bool {                // bool Return type listed
  return true;                        // C-style return statement
}

fn valuereturningfunc() -> f64 {      // f64 Return type listed
    println!("value returning function here");
    let pi;
    pi  = 3.1415926;
    pi                                // Tail expression
}

fn dummy(k : u32) {
    println!("k = { }", k);   // { } can contain formatting specifiers
}


fn main() {

    voidfunc();

    println!("pi = { }", valuereturningfunc());

    let mut k : u32 = 10;
    println!("k = { }", k);

    {
        let k : bool = true;          // Local k "shadows" mutable k
        println!("\tk = { }", k);     // C-style escape sequence for tab
    }

    k += 1;
    println!("k = { }", k);

    let n = 6;                  // What determines the data type of n ?
    dummy(n);                   // Could n be initialized above to -6 ?

    let mut m = 255;
    dummy(m);

    m += 1;
    dummy(m);

    println!("mystery = { }", mystery());
}
