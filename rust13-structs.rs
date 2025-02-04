//
// structfun  main.rs
//

struct Circle {
    x : f64,      // x-coordinate of circle center
    y : f64,      // y-coordinate of circle center
    r : f64       // radius of circle
}

impl Circle {               // Implement Circle functions

    fn new() -> Self {      // Initializes a new Circle similar to constructor
        Self {
            x : 0.0,
            y : 0.0,
            r : 0.0
        }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }

    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.r
    }

    fn get_center(&self) -> (f64, f64) {                // non-mutable self
        (self.x, self.y)
    }

    fn set_center(&mut self, xx : f64, yy : f64)  {    // mutable for modification of self
        self.x = xx;
        self.y = yy;
    }
}

fn main() {
    let mut sample : Circle = Circle::new();
    println!("area = { }", sample.area());
    println!("perimeter = { }", sample.perimeter());

    println!("**************************************");
    sample.x = 10.0;
    sample.y = 20.0;
    sample.r = 1.0;
    println!("area = { }", sample.area());
    println!("perimeter = { }", sample.perimeter());
    println!("centerX = { }   centerY = { }", sample.get_center().0, sample.get_center().1);

    println!("**************************************");
    sample.set_center(100.0, 100.0);
    println!("area = { }", sample.area());
    println!("perimeter = { }", sample.perimeter());
    println!("centerX = { }   centerY = { }", sample.get_center().0, sample.get_center().1);
}  
