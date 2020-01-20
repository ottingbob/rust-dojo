
fn main() {
  use std::mem;

  let color = "green";

  let print = || println!("`color`: {}", color);

  // call the closure using the borrow (&color)
  print();

  let _reborrow = &color;
  print();

  let _color_moved = color;

  let mut count = 0;
  let mut inc = || {
    count += 1;
    println!("`count`: {}", count);
  };

  // closure using mutable borrow
  inc();

  // closure still mutably borrows `count` because it is called later
  // an attempt to reborrow will error out
  inc();

  let _count_reborrorwed = &mut count;
  let movable = Box::new(3);
  let consume = || {
    println!("`moveable`: {:?}", movable);
    mem::drop(movable);
  };

  consume();

  let haystack = vec![1, 2, 3];

  // Using `move` forces closure to take ownership of
  // the captured variables
  // Removing `move` causes closure to borrow immutably
  // so haystack is still available
  // ---
  // let contains = move |needle| haystack.contains(needle);
  let contains = |needle| haystack.contains(needle);

  println!("{}", contains(&1));
  println!("{}", contains(&4));
}
