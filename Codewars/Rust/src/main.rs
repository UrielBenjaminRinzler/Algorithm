pub mod codewars;

pub use codewars::{codewars5};

fn main() {
    let result = codewars5::tests::count_duplicates("Indivisibilities");
    println!("result is {}", result);


}
