//
//  traitfun    main.rs
//

trait Area {                                  // Define Area trait
    fn get_area(&self) -> f64;
}

trait Perimeter {                             // Define Perimeter trait
    fn get_perimeter(&self) -> f64 { 98.6 }   // Default implementation of trait
}

struct Circle {
    x : f64,      // x-coordinate of circle center
    y : f64,      // y-coordinate of circle center
    r : f64       // radius of circle
}

impl Circle {

    fn new() -> Self {
        Self {
            x : 0.0,
            y : 0.0,
            r : 0.0
        }
    }

    fn get_radius(&self) -> f64 {
        self.r
    }

    fn set_radius(&mut self, rr : f64)  {
        self.r = rr;
    }

    fn get_center(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn set_center(&mut self, xx : f64, yy : f64)  {
        self.x = xx;
        self.y = yy;
    }
}

impl Area for Circle {        // Implement Area trait for Circle
    fn get_area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
}

impl Perimeter for Circle {   // Implement Perimeter trait for Circle
    fn get_perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.r
    }
}

// Generic functions accept any type T with Area or Perimeter trait defined respectively
fn area<T : Area>(item : &T) -> f64 {   
    item.get_area()
}    

fn perimeter<T : Perimeter>(item : &T) -> f64 {   
    item.get_perimeter()
}

impl Perimeter for i32 { }     // Implement trait for built-in type
                               // Rely on default implementation    

fn main() {

    println!("**************************************");

    let mut sample : Circle = Circle::new();
    println!("area = { }", area(&sample));
    println!("perimeter = { }", perimeter(&sample));
    println!("centerX = { }   centerY = { }    radius = { }", 
             sample.get_center().0, sample.get_center().1,
             sample.get_radius());

    println!("**************************************");
    sample.x = 10.0;
    sample.y = 20.0;
    sample.r = 1.0;
    println!("area = { }", area(&sample));
    println!("perimeter = { }", perimeter(&sample));
    println!("centerX = { }   centerY = { }    radius = { }", 
             sample.get_center().0, sample.get_center().1,
             sample.get_radius());
    println!("**************************************");

    sample.set_center(100.0, 100.0);
    sample.set_radius(10.0);
    println!("area = { }", area(&sample));
    println!("perimeter = { }", perimeter(&sample));
    println!("centerX = { }   centerY = { }    radius = { }", 
             sample.get_center().0, sample.get_center().1, 
             sample.get_radius());
    println!("**************************************");

    sample.set_center(100.0, 100.0);
    sample.set_radius(10.0);
    let (xx, yy) = sample.get_center();
    println!("area = { }", area(&sample));
    println!("perimeter = { }", perimeter(&sample));
    println!("centerX = { }   centerY = { }    radius = { }", xx, yy, sample.get_radius());
    println!("**************************************");

    let n : i32 = 5;
    println!("perimeter = { }", perimeter(&n));
    println!("**************************************");
}
