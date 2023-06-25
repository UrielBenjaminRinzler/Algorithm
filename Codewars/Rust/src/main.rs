pub mod codewars;

pub use codewars::{codewars2};

fn main() {
    let result = codewars2::tests::even_or_odd(12i32);
    println!("result is {}", result);
}
