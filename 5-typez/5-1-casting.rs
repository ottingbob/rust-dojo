// Supress all warnings from casts which overflow
#![allow(overflowing_literals)]

fn main() {
  let decimal = 65.4321_f32;

  // Error! no implicit conversion
  // let integer: u8 = decimal;

  // Explicit conversion
  let integer = decimal as u8;
  let character = integer as char;

  println!("Casting: {} -> {} -> {}", decimal, integer, character);

  // when casting any value to an unsigned type, T
  // std::T::MAX + 1 is added or subtracted until the value
  // fits into the new type

  // 1000 already fits in a u16
  println!("100 as a u16 is: {}", 1000 as u16);

  // 1000 - 256 - 256 - 256 = 232
  // Under the hood, the first 8 least significant bits (LSB) are kept,
  // while the rest towards the most significant bit (MSB) get truncated
  println!("1000 as a u8 is: {}", 1000 as u8);
  // -1 + 256 = 255
  println!("-1 as a u8 is: {}", (-1i8) as u8);

  // For positive number, this is the same as the modulus
  println!("1000 mod 256 is: {}", 1000 % 256);

  println!("128 as a i16 is: {}", 128 as i16);
  println!("128 as a i8 is: {}", 128 as i8);

  // repeating the example above
  println!("1000 as a u8 is: {}", 1000 as u8);
  println!("232 as a i8 is: {}", 232 as i8);

}
