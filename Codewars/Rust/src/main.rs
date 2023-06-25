pub mod codewars;

pub use codewars::{codewars1};

fn main() {
    let result = codewars1::tests::solution(10i32);
    println!("result is {}", result);
}
