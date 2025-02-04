//
// ownershipfun  main.rs
//

fn prettyprint(s : &String) {
    println!("s = { }", s);
}

fn mangle(s : &mut String) {            // mutable reference to String
    *s = "bye bye".to_string();
}

fn hit(s : &mut String) {
    s.insert(0, 'h');                   // dot is analogous to (*s).
    // (*s).insert(0, 'h');              // try this version - what do you see?
}

fn main() {
    let mut s1 : String = String::from("oops");
    println!("s1 = { }", s1);
    prettyprint(&s1);
    println!("s1 = { }", s1);
    hit(&mut s1);
    println!("s1 = { }", s1);
    mangle(&mut s1);
    println!("s1 = { }", s1);
}
