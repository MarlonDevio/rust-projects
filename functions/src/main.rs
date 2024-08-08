fn main() {
    println!("Hello, world!");
    another_function();

    let x = {
        let y = 5;
        y + 1
    };

    println!("The value of x is: {}", five());
}

fn another_function(){
    println!("Another function.");


}

fn five() -> i32 {
    5
}
