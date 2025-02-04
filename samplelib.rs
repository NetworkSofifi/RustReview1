//
// reviewreview02   samplelib.rs
//

pub fn somefunc(m : u32, n : u32) {  // Declare as a public function using pub
    for j in m..=n {                 // Must use ..= to include upper limit of range
        println!("{ } Hello world", j);
    }
}
