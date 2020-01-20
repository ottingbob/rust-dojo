
struct Point { x: i32, y: i32, z: i32 }

fn main() {
  let mut point = Point { x: 0, y: 0, z: 0 };

  let borrowed_point = &point;
  let another_borrow = &point;

  println!("Point has coordinates: ({}, {}, {})",
    borrowed_point.x, another_borrow.y, point.z);
  
  // Cant borrow point as mutable because its currently
  // borrowed as immutable.
  // let mutable_borrow = &mut point;

  println!("Point has coordinates: ({}, {}, {})",
    borrowed_point.x, another_borrow.y, point.z);
  
  let mutable_borrow = &mut point;

  mutable_borrow.x = 5;
  mutable_borrow.y = 2;
  mutable_borrow.z = 1;

  // cant borrow point as immutable because its already
  // borrowed as a mutable
  // let y = &point.y;

  // cant print because println! takes an immutable ref
  // println!("Point Z coordinate is {}", point.z);

  println!("Point has coordinates: ({}, {}, {})",
    mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

  let new_borrowed_point = &point;
  println!("Point has coordinates: ({}, {}, {})",
    new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);
}