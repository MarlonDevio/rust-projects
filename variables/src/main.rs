fn main() {
    // 1. COMPOUNDS

    // 1.1. tuples
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of x, y, z is: x:{} y:{} z:{}", x, y, z);
    println!("Check {x}");

    let five_hundred = tup.0;
    println!("The value of five_hundred is: {}", five_hundred);

    // 1.2. Arrays
    // on the stack
    
    // 2 Functions
    another_function();
}

fn another_function() {
    println!("Another function.");
}
