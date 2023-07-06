pub mod codewars;

pub use codewars::{codewars6};

fn main() {
    let result = codewars6::tests::get_count("abracadabra");
    println!("result is {}", result);


}
