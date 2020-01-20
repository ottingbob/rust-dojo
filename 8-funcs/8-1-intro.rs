
// no restriction on the order of function defs

fn main() {
  call_fn_later(100);
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
  if rhs == 0 {
    return false;
  }

  // expression so return is not needed
  lhs % rhs == 0
}

fn void_fn(n: u32) -> () {
  if is_divisible_by(n, 15) {
    println!("div by 15");
  } else if is_divisible_by(n, 3) {
    println!("div by 3");
  } else if is_divisible_by(n, 5) {
    println!("div by 5");
  } else {
    println!("{}", n);
  }
}

// void can also be used without `-> ()`
fn call_fn_later(n: u32) {
  for n in 1..n + 1 {
    void_fn(n);
  }
}
