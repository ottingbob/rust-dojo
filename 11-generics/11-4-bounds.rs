
use std::fmt::{Debug, Display};

trait HasArea {
  fn area(&self) -> f64;
}

impl HasArea for Rectangle {
  fn area(&self) -> f64 { self.length * self.height }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
#[allow(dead_code)]
struct Triangle { length: f64, height: f64 }

fn print_debug<T: Debug>(t: &T) {
  println!("{:?}", t);
}

fn area<T: HasArea>(t: &T) -> f64 { t.area() }

// Empty bounds example
struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// these funcs are only valid for types which implement
// these traits. empty traits are irrelevant
fn red<T: Red>(_: &T)   -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" } 

// multiple bounds example

fn compare_prints<T: Debug + Display>(t: &T) {
  println!("Debug: {:?}", t);
  println!("Display: {:?}", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
  println!("t: {:?}", t);
  println!("u: {:?}", u);
}

fn main() {
  let rectangle = Rectangle { length: 3.0, height: 4.0 };
  let _triangle = Triangle  { length: 3.0, height: 4.0 };

  print_debug(&rectangle);
  println!("Area: {}", area(&rectangle));

  // these will error because Triangle type does not implement either
  // Debug or HasArea

  let cardinal = Cardinal;
  let blue_jay = BlueJay;
  let _turkey  = Turkey;
  println!("A cardinal is {}", red(&cardinal));
  println!("A blue jay is {}", blue(&blue_jay));
  // println!("A turkey is neither {}", red(&_turkey));

  let string = "words";
  let array = [1, 2, 3];
  let vec = vec![1, 2, 3];

  compare_prints(&string);
  // compare_prints(&array);
  compare_types(&array, &vec);

}
