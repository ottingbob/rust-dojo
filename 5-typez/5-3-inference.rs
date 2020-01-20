fn main() {
  // Because of the annotation, the compiler knows that
  // `elem` has type u8
  let elem = 5u8;

  // Create an empty vector (a growable array)
  let mut vec = Vec::new();
  // At this point the compiler doesn't know of the exact
  // type of `vec` but it is of a `Vec<_?>`

  vec.push(elem);
  // compiler finds out `vec` is of type `u8`

  println!("{:?}", vec);
}