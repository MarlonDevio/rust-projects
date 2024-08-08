fn main() {
    // ownership
    {
        // string made up of 3 parts. pointer, length and capacity. Placed on the stack.
        // content is placed on the heap
        // capacity is the amount of memory allocated for the string
        // length is the amount of memory used by the string

        // if 2 variables point to the same memory location, and when they go out of scope,
        // the drop function gets called twice, which is a problem because it will try to free the same memory twice
        // this is called a double free error
        // copying pointer length and capacity without content is called a shallow copy.
        // Rust doesn't perform shallow copies, instead it invalidates the first variable.
        // This is called a move.

        // Rust will never automatically create a deep copy of your data.
        // Any copy will be considered inexpensive in terms of runtime performance.
        // To make a copy call copy method
        let s = String::from("I am valid");
        let s1 = s.clone();

        println!("{}", s);
        let s2 = s;
        // println!("{}", s); // This will give an error
    } // drop function gets called to return the memory

    {
        let y = 5;
        let x = y;
        println!("{}", y);
        println!("{}", x);
        // possible because integers have a fixed value and get copied and placed on
        // the stack
    }

    // String literal
    // let s = "hello";

    // String type
    let mut my_string = String::from("hello");

    // s.push_str(", world");
    //
    // println!("{s}");

    my_string.push_str(", world");
    println!("{}", my_string);
}
