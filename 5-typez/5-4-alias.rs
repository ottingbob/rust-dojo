
// `nsec` is a new name for `u64`
type NSec = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;

fn main() {
  let nanoseconds: NSec = 5 as u64_t;
  let inches: Inch = 2 as u64_t;

  println!("{} ns + {} in = {} unit?",
    nanoseconds,
    inches,
    nanoseconds + inches);
}