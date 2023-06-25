pub mod codewars;

pub use codewars::{codewars3};

fn main() {
    let result = codewars3::tests::get_char(65i32);
    println!("result is {}", result);
}
