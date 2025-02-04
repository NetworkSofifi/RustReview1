//
// scalarvariables  main.rs
//

fn main() {
    let i = 1;
    let j : i16 = 2;
    let k : u32 = 3;
    println!("i = { }", i);
    println!("j = { }", j);
    println!("k = { }", k);

    let mut m = 4i32;
    let n;
    n = 5_u32;
    println!("m = { }", m);
    println!("n = { }", n);

    let alpha : f32 = 1000.0;
    let beta = 10000.0;
    let charlie = 100_000.0;
    println!("alpha = { }", alpha);
    println!("beta = { }", beta);
    println!("charlie = { }", charlie);  
    
    m = 10; 
    println!("m = { }", m);
    println!("m = {m}");
    println!("m = {m:x}");
}
