
// use crate::deeply::nested::{
//   my_first_function,
//   my_second_function,
//   AndATraitType
// };

use deeply::nested::function as other_function;

fn function() {
  println!("called function()");
}

mod deeply {
  pub mod nested {
    pub fn function() {
      println!("called deeply::nested::function()");
    }
  }
}

fn main() {
  // my_first_function();

  other_function();

  println!("Entering block");
  {
    use crate::deeply::nested::function;
    function();

    println!("Leaving block");
  }

  function();
}