

/*

// Awkward matching enum:

let optional = Some(7);

match optional {
  Some(i) => {
    println!("This is a really long string and `{:?}`", i);
  },
  _ => {},
};

*/

enum Foo {
  Bar,
  Baz,
  Qux(u32)
}

#[allow(dead_code)]
enum Tester { 
  DefaultTester,
  MockTester
}

fn main() {
  let number = Some(7);
  let letter: Option<i32> = None;
  let emoticon: Option<i32> = None;

  if let Some(i) = number {
    println!("Matched {:?}!", i);
  }

  if let Some(i) = letter {
    println!("Matched {:?}!", i);
  } else {
    println!("Didn't match a number. Let's go with a letter!");
  }

  // Provide and altered failing condition
  let i_like_letters = false;

  if let Some(i) = emoticon {
    println!("Matched {:?}!", i);
  } else if i_like_letters {
    println!("Didnt match a number -- go with a letter");
  } else {
    println!("no letters go emoji");
  }

  // Example Enum
  let a = Foo::Bar;
  let b = Foo::Baz;
  let c = Foo::Qux(100);

  // Variable a matches Foo::Bar
  if let Foo::Bar = a {
    println!("a is foobar");
  }

  // Variable b does not match Foo::Bar
  // So this will print nothing
  if let Foo::Bar = b {
    println!("b is foobar");
  }

  // Variable c matches Foo::Qux which has a value
  // Similar to Some() in the previous example
  if let Foo::Qux(value) = c {
    println!("c is {}", value);
  }

  // Binding also works with `if let`
  if let Foo::Qux(value @ 100) = c {
    println!("c is one hunten (value=[{}])", value);
  }

  let tester = Tester::DefaultTester;

  if let Tester::DefaultTester = tester {
    println!("default tester");
  }
}
