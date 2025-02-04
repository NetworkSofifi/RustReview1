//
// mathlibproject  main.rs
//

mod customlib;
use customlib::square;

fn main() {
    for k in 1..=10 {
        println!("The square of { } is { }", k, square(k));
    }
}
