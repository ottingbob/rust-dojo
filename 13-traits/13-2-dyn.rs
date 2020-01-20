
struct Sheep {}
struct Cow {}

trait Animal {
  fn noise(&self) -> &'static str;
}

impl Animal for Sheep {
  fn noise(&self) -> &'static str {
    "beh"
  }
}

impl Animal for Cow {
  fn noise(&self) -> &'static str {
    "muu"
  }
}

// returns some struct of animal but we dont know which 
// one at compile time
fn random_animal(random_number: f64) -> Box<dyn Animal> {
  if random_number < 0.5 {
    Box::new(Sheep {})
  } else {
    Box::new(Cow {})
  }
}

fn main() {
  let random_number = 0.534;
  let animal = random_animal(random_number);
  println!("Youve randomly chosen an animal that seys: {}", animal.noise());
}
