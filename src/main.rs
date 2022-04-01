use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let num: i32 = args[1].parse().unwrap();
    let perfect_square = find_perfect_sqare(num);
    let multiple = num / perfect_square;
    let multiple: f64 = f64::from(multiple).sqrt();
    println!("sqrt({}) * {} == sqrt({})", perfect_square, multiple, num)
}
#[test]
fn is() {
  assert!( is_perfect_square(4));
  assert!(! is_perfect_square(2));
  assert!(! is_perfect_square(8));
  assert!( is_perfect_square(16));
}
#[test]
fn find(){
  assert_eq!(find_perfect_sqare(8), 2);
  assert_eq!(find_perfect_sqare(132), 33);
  assert_eq!(find_perfect_sqare(325), 13);
  assert_eq!(find_perfect_sqare(136), 34);
}
fn is_perfect_square(n: i32) -> bool{
    let n: f64 = f64::from(n);
    n > 0.0 && n.sqrt() % 1.0 == 0.0
}
fn find_perfect_sqare(num: i32) -> i32{
  let mut divisor: i32 = 1;
  loop {
    divisor += 1;
    if num % divisor == 0 &&  is_perfect_square(num / divisor){
	    return divisor
		}
    if divisor > 1000{
      panic!("infinite loop!")
    }
  }
}