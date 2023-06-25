pub mod codewars;

pub use codewars::{codewars4};

fn main() {
    let result = codewars4::tests::find_average(&[17.0, 16.0, 16.0, 16.0, 16.0, 15.0, 17.0, 17.0, 15.0, 5.0, 17.0, 17.0, 16.0, ]);
    println!("result is {}", result);
    assert_eq!(result, 200.0 / 13.0);
}
