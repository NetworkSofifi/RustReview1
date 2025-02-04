//
// arrayfun  main.rs
//

fn main() {
    let even_numbers : [u32; 6] = [0, 2, 4, 6, 8, 10];
    for k in 0..6 {
        println!("even_number[{ }] is { }", k, even_numbers[k]);
    }
    
    let flags = [false; 5];
    for j in 0..5 {
        println!("flags[{ }] = { }", j, flags[j]);
    }
    
    let mut somearray : [i32; 10] = [0; 10];
    let mut count = 0_i32;
    for m in 0..10 {
        somearray[m] = count;
        count = count + 1;
        println!("somearray[{ }] = { }", m, somearray[m]);
    }
}
