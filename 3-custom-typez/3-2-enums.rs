
enum WebEvent {
  // An `enum` may either be `unit-like`
  PageLoad,
  PageUnload,
  // like tuple structs
  KeyPress(char),
  Paste(String),
  // or c-like structures.
  Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing
fn inspect(event: WebEvent) {
  match event {
    WebEvent::PageLoad => println!("page loaded"),
    WebEvent::PageUnload => println!("page unloaded"),
    // Destructure `c` from inside the `enum`.
    WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
    WebEvent::Paste(s) => println!("pasted \"{}\".", s),
    // Destructure `Click` into `x` and `y`.
    WebEvent::Click { x, y } => {
      println!("clicked at x={}, y={}.", x, y);
    },
  }
}

#[derive(Debug)]
enum VerVerboseEnumOfThingsToDoWithNumbers {
  Add,
  Subtract,
}

// Create a type alias
type Operations = VerVerboseEnumOfThingsToDoWithNumbers;

// Using an enum
enum Status {
  Rich,
  Poor,
}

enum Work {
  Civilian,
  Soldier
}

// C-Style zyntax
#![allow(dead_code)]
enum Number {
  Zero,
  One,
  Two,
}

// enum with explicit discriptor
enum Color {
  Red   = 0xff0000,
  Green = 0x00ff00,
  Blue  = 0x0000ff,
}

fn main() {
  let pressed = WebEvent::KeyPress('x');
  // `to_owned()` creates an owned `String` from a string slice.
  let pasted = WebEvent::Paste("my text".to_owned());
  let click  = WebEvent::Click { x: 20, y: 80 };
  let load   = WebEvent::PageLoad;
  let unload = WebEvent::PageUnload;

  inspect(pressed);
  inspect(pasted);
  inspect(click);
  inspect(load);
  inspect(unload);

  // Refer to each variant via an alias
  println!("{:?} {:?}", Operations::Add, Operations::Subtract);

  // Explicitly `use` each name so they are avilable without 
  // manual scoping
  use crate::Status::{Poor, Rich};
  // Automatically `use` each name inside `Work`.
  use crate::Work::*;

  // Equivalent to `Status::Poor`
  #[allow(unused_assignments)]
  let mut status = Poor;
  status = Rich;
  
  // Equivalent to `Work::Civilian`
  #[allow(unused_assignments)]
  let mut work = Civilian;
  work = Soldier;

  match status {
    // Note the lack of scoping because of the explicit `use` above.
    Rich => println!("The rich have lots of money!"),
    Poor => println!("The poor have no money..."),
  }

  match work {
    // Note again the lack of scoping
    Civilian => println!("Civilians work!"),
    Soldier  => println!("Soldiers fight!"), 
  }

  // `enums` can be cast as integers
  println!("zero is {}", Number::Zero as i32);
  println!("one is {}", Number::One as i32);

  println!("roses are #{:06x}", Color::Red as i32);
  println!("violoets are #{:06x}", Color::Blue as i32);
}
