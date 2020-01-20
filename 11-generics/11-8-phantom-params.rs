
use std::marker::PhantomData;

// Phantom tuple struct which is a generic over A with
// hidden parameter B

// Allow equality test for this type
#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

// phantom struct which is generic over A with hidden param B
#[derive(PartialEq)]
struct PhantomStruct<A, B> { first: A, phantom: PhantomData<B> } 

// Storage is allocated for A but not B
// So B cannot be used in computations

fn main() {
  let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
  let _tuple1: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

  let _struct1: PhantomStruct<char, f32> = PhantomStruct {
    first: 'Q',
    phantom: PhantomData,
  };

  let _struct1: PhantomStruct<char, f64> = PhantomStruct {
    first: 'Q',
    phantom: PhantomData,
  };

  // Compile error -- type mismatch cannot be compared
  // println!("{}", _tuple1 == _tuple2);

  // Compile error -- type mismatch cannot be compared
  // println!("{}", _struct1 == _struct2);
}
