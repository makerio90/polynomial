use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let num: f64 = args[1].parse().unwrap();
    let perfect_square = find_perfect_sqare(num);
    let multiple = (num / perfect_square).sqrt();
    println!("sqrt({}) * {} == sqrt({})", perfect_square, multiple, num)
}
#[test]
fn is() {
  assert!( is_perfect_square(4.0));
  assert!(! is_perfect_square(2.0));
  assert!(! is_perfect_square(8.0));
  assert!( is_perfect_square(16.0));
}
#[test]
fn find(){
  assert_eq!(find_perfect_sqare(8.0), 2.0);
  assert_eq!(find_perfect_sqare(132.0), 33.0);
  assert_eq!(find_perfect_sqare(325.0), 13.0);
  assert_eq!(find_perfect_sqare(136.0), 34.0);
}
fn is_perfect_square(n: f64) -> bool{
    n > 0.0 && n.sqrt() % 1.0 == 0.0
}
fn find_perfect_sqare(num: f64) -> f64{
  let mut divisor: f64 = 1.0;
  loop {
    divisor += 1.0;
    if num % divisor == 0.0 &&  is_perfect_square(num / divisor){
	    return divisor
		}
    if divisor > 100.0{
      panic!("infinite loop!")
    }
  }
}