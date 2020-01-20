
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
  println!("x is {} and y is {}", x, y);
}

fn failed_borrow<'a>() {
  let _x = 12;

  // A short lifetime cannot be coerced into a longer one
}

fn main() {
  let (four, nine) = (4, 9);

  print_refs(&four, &nine);

  failed_borrow();
}