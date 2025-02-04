//
// stringfun  main.rs
// 

const  _SMILEYFACE : char = '\u{1F600}';

fn main() {
    let snack = "burger";    // <=  &str type 
    let beverage : &str = "soda";
    let mut mystring : String = "Homer".to_string();
    let yourstring : String = String::from("Simpson");

    println!("\n");
    println!("mystring = { }", mystring);
    println!("yourstring = { }", yourstring);
    println!("snack = { }", snack);
    println!("beverage = { }", beverage);

    println!("\n");
    println!("{ } == { } ==> { }", mystring, yourstring, mystring == yourstring);
    println!("{ } != { } ==> { }", mystring, yourstring, mystring != yourstring);
    println!("{ } < { } ==> { }", mystring, yourstring, mystring < yourstring);
    println!("{ } > { } ==> { }", mystring, yourstring, mystring > yourstring);

    println!("\n");
    mystring.push(' ');              // append single space character
    mystring.push_str(snack);        // append borrowed string slice
    println!("mystringsnack = { }", mystring);

    mystring.push('_'); 
    mystring.push_str(beverage); 
    println!("mystringsnackbeverage = { }", mystring);

    mystring.push(' '); 
    mystring.push(_SMILEYFACE);       // append UTF-8 smiley face
    println!("mystringsnackbeveragehappy = { }", mystring);
}
