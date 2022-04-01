pub mod tests;
pub mod square_finders;
use std::env;
use crate::square_finders::find_perfect_square;
fn main() {
    let args: Vec<String> = env::args().collect();
    let num: i32 = args[1].parse().unwrap();
    let perfect_square = find_perfect_square(num);
    let multiple = num / perfect_square;
    let multiple: f64 = f64::from(multiple).sqrt();
    println!("sqrt({}) * {} == sqrt({})", perfect_square, multiple, num)
}