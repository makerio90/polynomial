use crate::square_finders::{find_perfect_square, is_perfect_square};

#[test]
fn is() {
  assert!( is_perfect_square(4));
  assert!(! is_perfect_square(2));
  assert!(! is_perfect_square(8));
  assert!( is_perfect_square(16));
}
#[test]
fn find(){
  assert_eq!(find_perfect_square(8), 2);
  assert_eq!(find_perfect_square(132), 33);
  assert_eq!(find_perfect_square(325), 13);
  assert_eq!(find_perfect_square(136), 34);
}