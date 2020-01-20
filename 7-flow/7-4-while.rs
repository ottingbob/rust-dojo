
fn main() {
  // Counter var
  let mut n = 1;

  // loop while `n` is lt 101
  while n < 101 {
    if n % 15 == 0 {
      println!("zero");
    } else if n % 3 == 0 {
      println!("mod 3");
    } else if n % 5 == 0 {
      println!("mod funf");
    } else {
      println!("{}", n);
    }

    // Increment counter
    n += 1;
  }
}
