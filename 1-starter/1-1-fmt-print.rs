fn main() {
  println!("{} days", 31);

  // Without a suffix, 31 becomes an i32. You can change what type 31 is
  // by providing a suffix

  // There are various optional patterns this works with. Positional
  // arguments can be used.
  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

  // As can named arguments
  println!("{subject} {verb} {object}",
    object="the lazy dog",
    subject="the quick brown fox",
    verb="jumps over");
  
  // Special formatting can be specified after a `:`
  println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

  // Right alignment for a specified width
  println!("{number:>width$}", number=1, width=6);

  // You can pad numbers with extra zeros
  println!("{number:0width$}", number=1, width=6);

  // Rust even checks to make sure the correct number of arguments are used
  println!("My name is {0}, {1} {0}", "Bond", "James");

  // Create a structure named `Structure` which contains an `i32`
  #[allow(dead_code)]
  struct Structure(i32);

  // However, custom types such as this structure require more complicated
  // handling. This will not work.
  // println!("This struct `{}` won't print...", Structure(3));

}