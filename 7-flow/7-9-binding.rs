
fn age() -> u32 {
  15
}

fn some_number() -> Option<u32> {
  Some(42)
}

fn main() {

  println!("Tell me what type of person you are");

  match age() {
    0 => println!("You are not born at 0 yrs old"),
    n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
    n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
    n => println!("I'm an adult of age {:?}", n),
  }

  match some_number() {
    Some(n @ 42) => println!("The Answer: {}!", n),
    Some(n) => println!("Not interesting ... {}", n),
    _ => (),
  }
}