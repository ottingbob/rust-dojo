
pub fn public_function() {
  println!("called libs public_funciton()");
}

fn private_function() {
  println!("called libs private_function()");
}

pub fn indirect_access() {
  print!("called libs indirect_access(), that\n> ");
  private_function();
}
