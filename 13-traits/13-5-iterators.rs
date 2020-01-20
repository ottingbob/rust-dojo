
struct Fibonacci {
  curr: u32,
  next: u32,
}

impl Iterator for Fibonacci {
  type Item = u32;

  fn next(&mut self) -> Option<u32> {
    let new_next = self.curr + self.next;

    self.curr = self.next;
    self.next = new_next;

    Some(self.curr)
  }
}

fn fib() -> Fibonacci {
  Fibonacci { curr: 0, next: 1 }
}

fn main() {
  let mut sequence = 0..3;

  println!("Four consecutive `next` calls on 0..3");
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());
  println!("> {:?}", sequence.next());

  println!("Iterate through 0..3 using `for`");
  for i in 0..3 {
    println!("> {}", i);
  }

  println!("The first four terms of the fib sequence are: ");
  for i in fib().take(4) {
    println!("> {}", i);
  }

  println!("The next four terms of the fib sequence are: ");
  for i in fib().skip(4).take(4) {
    println!("> {}", i);
  }

  let array = [1u32, 3, 3, 7];

  println!("Iterate the following array {:?}", &array);
  for i in array.iter() {
    println!("> {}", i);
  }
}
