pub fn is_perfect_square(n: i32) -> bool{
    let n: f64 = f64::from(n);
    n > 0.0 && n.sqrt() % 1.0 == 0.0
}
pub fn find_perfect_square(num: i32) -> i32{
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