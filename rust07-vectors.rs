//
// vectorfun  main.rs
//

fn main() {
    let mut v : Vec<u32> = Vec::new();

    println!("\nUpon Creation");
    println!("\tvector length = { }", v.len());
    println!("\tvector capacity = { }", v.capacity());

    v.push(8);
    v.push(2);
    println!("After 2 push operations");
    println!("\tvector length = { }", v.len());
    println!("\tvector capacity = { }", v.capacity()); 
 
    v.push(4);
    v.push(6);
    println!("After 4 push operations");
    println!("\tvector length = { }", v.len());
    println!("\tvector capacity = { }", v.capacity());   

    println!("**** Access via indexing ****");
    for k in 0..v.len() {
        println!("v[{ }] = { }", k, v[k]);
    }

    println!("**** Access via iterator ****");
    for k in &v {          
        println!("{ }", k);
    }

    v.pop();
    println!("After 1 pop operation");
    println!("\tvector length = { }", v.len());
    println!("\tvector capacity = { }", v.capacity());

    v.pop();
    println!("After 2 pop operations");
    println!("\tvector length = { }", v.len());
    println!("\tvector capacity = { }", v.capacity());

    println!("********");
    for k in &v {
        println!("{ }", k);
    }
    println!("********");

    println!("Remove value at index 0");
    v.remove(0);

    println!("Insert value 4 at index 1");
    v.insert(1, 4);

    println!("********");
    for k in &v {
        println!("{ }", k);
    }
    println!("********");

    let mut w = vec![2_u32, 4, 6, 8];
    for j in 0..w.len() {
        println!("w[{ }] = { }", j, w[j]);
    }

    // let u = vec![1, 3.14159];    // Try uncommenting this line
}
