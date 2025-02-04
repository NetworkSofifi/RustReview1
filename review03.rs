//
// rustreview03  main.rs
//

fn main() {
    let mut myarray : [i32; 6];      // mutable array so we can alter the array contents

    myarray = [5, 4, 3, 2 ,1, 0];    // Separated declaration and initialization

    println!("******************");
    for k in 0..6 {                  // Beware of index limit!  Exclude the range endpoint  
        println!("myarray[{ }] = { }", k, myarray[k]);
        myarray[k] += 1;
    }

    println!("******************"); 
    let mut myvec : Vec<i32> = Vec::new();

    println!("\tvector length = { }", myvec.len());
    println!("\tvector capacity = { }", myvec.capacity());

    for k in 0..6 {
        myvec.push(myarray[k]);    // Add each array element to vector
        println!("myvec[{ }] = { }", k, myvec[k]);
    }
    
    println!("\tvector length = { }", myvec.len());
    println!("\tvector capacity = { }", myvec.capacity());
    println!("\tfirst = { }", myvec[0]);
    println!("\t last = { }", myvec[myvec.len()-1]);

    println!("******************");
}
