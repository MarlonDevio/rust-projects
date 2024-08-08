mod my_module;
use crate::my_module::submod1::hello_from_submod1;
use crate::my_module::submod2::hello_from_submod2;

fn main() {
    println!("Hello, world!");
    hello_from_submod2();
    hello_from_submod1();
}
