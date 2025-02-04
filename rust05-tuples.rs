//
// tuplefun  main.rs
//

fn main() {
    let value = (1, 3.14156);
    println!("value.0 = { }", value.0);
    println!("value.1 = { }", value.1);  

    let point : (u32, u32) = (3, 4);
    let xcomponent = point.0;
    let ycomponent = point.1;
    let (x, y) = point;
    println!("point = ({ }, { })", point.0, point.1);
    println!("point = ({ }, { })", xcomponent, ycomponent);
    println!("(x,y) = ({ }, { })", x, y);
    println!("(x,y) = ({x}, {y})");

    let rgb : (u8, u8, u8) = (0, 255, 0);
    let pixel : ((u32, u32), (u8, u8, u8)) = (point, rgb);
    let r = pixel.1.0;
    let g = pixel.1.1;
    let b = pixel.1.2;
    let (red, green, blue) = pixel.1;
    println!("(red, green, blue) = ({ }, { }, { })", red, green, blue);
    println!("(r, g, b) = ({ }, { }, { })", r, g, b);
    println!("(x, y) = ({ }, { })", pixel.0.0, pixel.0.1);
}
