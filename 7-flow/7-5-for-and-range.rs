
fn main() {
  // `n` will take the values: 1, 2, ..., 100 in each iteration
  for n in 1..101 {
    if n % 15 == 0 {
      println!("{} is divisible by 15", n);
    }
  }

  let names = vec!["Bob", "Frank", "Ferris"];

  // names.iter() does not modify collection
  // names.into_iter() consumes the collection and not for reuse
  //    later on in the loop
  // names.iter_mut() borrows eaach elemt of the collection 
  //     allowing it to be modified in place
  for name in names.iter() {
    match name {
      &"Ferris" => println!("There is a rusty-bya among us.."),
      _ => println!("Hello yng bya {}", name),
    }
  }
}