use std::io;

fn main() {
    println!("Hello, world!");


    loop {

        let mut fahrenheit = String::new();

        println!("Enter the fahrenheit that needs to be converted to Celsiuis");
        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Enter a valid number");

        let fahrenheit: f64 = match fahrenheit.trim().parse(){
            Ok(fahr) => fahr,
            Err(_) => {
                println!("No valid number given");
                continue;
            }
        };

        let celsius = fahrenheit_to_celsius(fahrenheit);
        println!("{fahrenheit} fahrenheit is equal to {celsius} celsius");
        break;
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 1.8
}
