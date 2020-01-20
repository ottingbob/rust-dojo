
fn destroy_box(c: Box<i32>) {
  println!("Destroying a box that contains {}", c);
}

fn main() {
  let x = 5u32;

  let y = x;

  println!("x is {}, and y is {}", x, y);

  let a = Box::new(5i32);

  println!("a contains: {}", a);

  // b now owns the data although both are pointers
  let b = a;
  destroy_box(b);

  // Mutability example
  let immutable_box = Box::new(5u32);

  println!("immutable_box contains {}", immutable_box);

  let mut mutable_box = immutable_box;
  println!("mutable_box contains {}", mutable_box);

  *mutable_box = 4;

  println!("mutable_box now contains {}", mutable_box);
}
