//
// loopfun  main.rs
//

fn main() {
   
    // WHILE loop
    let mut i : u32 = 0;
    while i < 5 {
        i = i + 1;
        println!("i = { }", i);
    }
    println!("******");

    // Infinite loop mimics WHILE
    let mut j : u32 = 0;
    loop {
        if j == 5 {
            break;
        }

        j = j + 1;
        println!("j = { }", j);
    }
    println!("******"); 

    // Infinite loop mimics DO-WHILE
    let mut k : u32 = 0;
    loop {
        k = k + 1;
        println!("k = { }", k);

        if k == 5 {
            break;
        }
    }
    println!("******");

    // Labels and nested loops
    let mut x : u32 = 0;
    let mut y : u32 = 0;

    'outerloop:  loop {
        println!("outerloop");
        x = x + 1;

        'innerloop:  loop {
            println!("innerloop");
            y = y + 1;

            println!("({ }, { })", x, y);

            if x > 3 {
                println!("BREAK - outerloop");
                break 'outerloop;
            }

            if y > 2 {
                println!("BREAK - innerloop");
                break 'innerloop;
            }
        }
    }
}
