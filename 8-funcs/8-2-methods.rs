
struct Point {
  x: f64,
  y: f64,
}

// all `Point` methods go in here
impl Point {
  // static method -- usually used as constructors
  fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
  }

  // Another static method, taking two arguments:
  fn new(x: f64, y: f64) -> Point {
    Point { x: x, y: y }
  }
}

struct Rectangle {
  p1: Point,
  p2: Point,
}

impl Rectangle {
  // instance method
  // `&self` is shorthand for `self: &Self`, where `Self` is
  // the type of the caller. In this case `Self` = `Rectangle`
  fn area(&self) -> f64 {
    let Point { x: x1, y: y1 } = self.p1;
    let Point { x: x2, y: y2 } = self.p2;

    // `abs` is a `f64` method that returns the absolute
    // value of the caller
    ((x1 - x2) * (y1 - y2)).abs()
  }

  fn perimeter(&self) -> f64 {
    let Point { x: x1, y: y1 } = self.p1;
    let Point { x: x2, y: y2 } = self.p2;

    2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
  }

  // This method requires the caller object to be mutable
  // `&mut self` desugars to `self: &mut Self`
  fn translate(&mut self, x: f64, y: f64) {
    self.p1.x += x;
    self.p2.x += x;

    self.p1.y += y;
    self.p2.y += y;
  }
}

// `Pair` owns resources: two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
  // this method "consumes" resources fo caller object
  // `self` is `self: Self`
  fn destroy(self) {
    let Pair(first, second) = self;

    println!("Destroying Pair({}, {})", first, second);
  }
}

fn main() {
  let rectangle = Rectangle {
    // static method calls with double colons
    p1: Point::origin(),
    p2: Point::new(3.0, 4.0),
  };

  println!("Rectangle perimeter: {}", rectangle.perimeter());
  println!("Rectangle area: {}", rectangle.area());

  let mut square = Rectangle {
    p1: Point::origin(),
    p2: Point::new(1.0, 1.0),
  };

  square.translate(1.0, 1.0);

  let pair = Pair(Box::new(1), Box::new(2));

  pair.destroy();
}
