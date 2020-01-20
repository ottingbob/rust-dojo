
mod my {
  pub struct OpenBox<T> {
    pub contents: T,
  }

  #[allow(dead_code)]
  pub struct ClosedBox<T> {
    contents: T,
  }

  impl<T> ClosedBox<T> {
    pub fn new(contents: T) -> ClosedBox<T> {
      ClosedBox {
        contents: contents,
      }
    }
  }
}

fn main() {
  let open_box = my::OpenBox { contents: "public information" };

  println!("The open box contains: {}", open_box.contents);

  // public struct but cannot access private field
  // let closed_box = my::ClosedBox { contents: "classified" }

  // can access private fields with public constructor
  let _closed_box = my::ClosedBox::new("classified");

  // private fields of pubic struct cannot be accessed
  // println!("The closed box contains: {}", _closed_box.contents);
}
