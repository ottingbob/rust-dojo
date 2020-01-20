
fn main() {
  // let pair = (2, -2);
  let pair = (2, 13);

  println!("Tell me about {:?}", pair);
  match pair {
    (x, y) if x == y => println!("same value"),
    (x, y) if x + y == 0 => println!("oppisite values"),
    (x, _) if x % 2 == 1 => println!("x is odd"),
    _ => println!("No matches!"),
  }
}