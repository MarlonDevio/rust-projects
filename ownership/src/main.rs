fn main() {
    // Borrowing -> action of creating a reference
    // References and borrowing
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    // only 1 reference to a mutable value is allowed!
    let mut s2 = String::from("Another Hello");
    change(&mut s2);

    println!("{s2}");
    println!("The size of '{s1}' is {len}.");

    // solution for multiple references
    {
        // new scope
        let r1 = &mut s2;
        println!("{r1}");
    }
    {
        let r2 = &s2;
        let r3 = &s2;
        println!("{r2} {r3}");
        println!("{r2} {r3}");
    }
    let r2 = &mut s2;
    println!("{r2}")
}

// not taking ownership
// value it points to will not be dropped when reference stops being used
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String){
    some_string.push_str(", World");
}