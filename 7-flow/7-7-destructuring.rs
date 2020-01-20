
// We dont use all the enums
#[allow(dead_code)]
enum Color {
  // These 3 are specified by their name
  Red,
  Blue,
  Green,
  // This tie u32 tuples to named color models
  RGB(u32, u32, u32),
  HSV(u32, u32, u32),
  HSL(u32, u32, u32),
  CMY(u32, u32, u32),
  CMYK(u32, u32, u32, u32),
}

fn main() {
  // Tuples
  let pair = (0, -2);
  println!("Tell me about {:?}", pair);
  match pair {
    // Destructure the second
    (0, y) => println!("First is `0` and `y` is `{:?}`", y),
    (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
    _      => println!("It doesn't matter what they are"),
    // `_` means don't bind the value to a variable
  }

  // Enum
  let color = Color::RGB(122, 17, 40);
  println!("What colour is it?");
  match color {
    Color::Red => println!("The color is Red!"),
    Color::Blue => println!("the color is Blue!"),
    Color::Green => println!("The color is Green!"),
    Color::RGB(_r, _g, _b) => println!("RGB"),
    Color::HSV(_r, _g, _b) => println!("RGB"),
    Color::HSL(_r, _g, _b) => println!("RGB"),
    Color::CMY(_r, _g, _b) => println!("RGB"),
    Color::CMYK(_r, _g, _b, _a) => println!("RGB"),
  }

  // Pointers && ref
  // Assign a reference of type `i32` the `&` tells
  // us a reference is being assigned
  let reference = &4;

  match reference {
    &val => println!("Go a value via destructuring {:?}", val),
  }

  match *reference {
    val => println!("Got a value via dereferencing: {:?}", val),
  }

  let _not_a_reference = 3;
  let ref _is_a_reference = 3;

  let value = 5;
  let mut mut_value = 6;

  // create the reference to value
  match value {
    ref r => println!("Gat a reference to the value: {:?}", r),
  }

  match mut_value {
    ref mut m => {
      // got a reference to defreference it
      *m += 10;
      println!("We added 10. `mut_value`: {:?}", m);
    },
  }

  // Structs
  struct Foo {
    x: (u32, u32),
    y: u32,
  }

  // Try changing the values in the struct to see what happens
  let foo = Foo { x: (1, 2), y: 3 };

  match foo {
    Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {} ", b, y),

    // destructure and rename vars -- order doesnt matter
    Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

    Foo { y, .. } => println!("y = {}, we dont care about x", y),
  }
}
