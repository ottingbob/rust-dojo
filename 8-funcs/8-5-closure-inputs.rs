
fn apply<F>(f: F) where
  F: FnOnce() {
    f();
}

fn apply_to_3<F>(f: F) -> i32 where
  F: Fn(i32) -> i32 {
    f(3)
}

fn main() {
  use std::mem;

  let greeting = "hello";
  // `to_owned` creates owned data from borrowed one
  let mut farewell = "goodbye".to_owned();

  // Capture 2 variables: `greeting` by reference and
  // `farewell` by value
  let diary = || {
    println!("I said {}.", greeting);

    farewell.push_str("!!!");
    println!("Then I screamed {}.", farewell);
    println!("Now I can sleep. zzzz");

    // manually calling `mem::drop` forces
    // `farewell` to be captured by value.  
    // Now requires `FnOnce`
    mem::drop(farewell);
  };

  // Call the function which applies to closure
  apply(diary);

  // `double` satisfies `apply_to_3`s trait bound
  let double = |x| 2 * x;

  println!("3 doubled: {}", apply_to_3(double));

}
